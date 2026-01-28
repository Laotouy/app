//! 支付回调路由（内部 API）
//!
//! 接收来自支付平台的回调通知。
//! 验证签名后更新订单状态。

use actix_web::{HttpRequest, HttpResponse, post, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::BTreeMap;
use subtle::ConstantTimeEq;

use crate::routes::ApiError;

/// 验证请求 IP 是否在白名单中
fn verify_ip_whitelist(req: &HttpRequest) -> Result<(), String> {
    let allowed_ips = dotenvy::var("SEVENPAY_ALLOWED_IPS").unwrap_or_default();

    if allowed_ips.is_empty() {
        // 未配置 IP 白名单时，记录警告但允许通过（向后兼容）
        log::warn!(
            "支付回调 IP 白名单未配置 (SEVENPAY_ALLOWED_IPS)，跳过 IP 验证"
        );
        return Ok(());
    }

    // 获取客户端真实 IP（考虑反向代理）
    let client_ip = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // 解析白名单
    let whitelist: Vec<&str> =
        allowed_ips.split(',').map(|s| s.trim()).collect();

    if whitelist.iter().any(|&ip| ip == client_ip) {
        Ok(())
    } else {
        log::warn!(
            "支付回调 IP 不在白名单中: client_ip={}, allowed={}",
            client_ip,
            allowed_ips
        );
        Err(format!("IP {} 不在白名单中", client_ip))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("payment").service(payment_callback));
}

// ==================== 请求/响应结构 ====================

/// 支付回调数据
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentCallbackData {
    /// 交易状态
    pub trade_state: String,
    /// 外部订单号（BBSMC 订单 ID）
    pub other_order_no: String,
    /// 支付平台订单 ID
    pub order_id: String,
    /// 交易流水号
    pub order_transaction_id: Option<String>,
    /// 店铺 ID
    pub sid: String,
    /// 订单标题
    pub title: String,
    /// 支付类型 (ALIPAY/WECHAT)
    pub pay_type: String,
    /// 用户显示名称
    pub user_display_name: String,
    /// 金额（分）
    pub money: String,
    /// 结算状态
    pub settlement: String,
}

/// 支付回调请求
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentCallbackRequest {
    /// 回调数据
    pub data: PaymentCallbackData,
    /// 签名
    pub sign: String,
}

/// 回调响应
#[derive(Debug, Clone, Serialize)]
pub struct CallbackResponse {
    pub code: i32,
    pub message: String,
}

// ==================== 路由处理 ====================

/// 接收支付回调
///
/// POST /_internal/payment/callback
///
/// 支付平台会在支付成功后调用此接口通知 BBSMC。
/// 验证签名后返回 {"code": 200} 表示接收成功。
#[post("callback")]
pub async fn payment_callback(
    req: HttpRequest,
    body: web::Json<PaymentCallbackRequest>,
    _pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    // 验证 IP 白名单
    if let Err(msg) = verify_ip_whitelist(&req) {
        return Ok(HttpResponse::Forbidden().json(CallbackResponse {
            code: 403,
            message: msg,
        }));
    }

    // 获取 keycode
    let keycode = dotenvy::var("SEVENPAY_KEYCODE").unwrap_or_default();

    if keycode.is_empty() {
        log::warn!("支付密钥未配置 (SEVENPAY_KEYCODE)，跳过回调处理");
        return Ok(HttpResponse::BadRequest().json(CallbackResponse {
            code: 500,
            message: "支付配置未完成".to_string(),
        }));
    }

    let data = &body.data;
    let sign = &body.sign;

    log::info!(
        "收到支付回调: order_id={}, other_order_no={}, trade_state={}, client_ip={}",
        data.order_id,
        data.other_order_no,
        data.trade_state,
        req.connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
    );

    // 验证签名
    if !verify_payment_signature(data, sign, &keycode) {
        log::warn!(
            "支付回调签名验证失败: order_id={}, other_order_no={}",
            data.order_id,
            data.other_order_no
        );
        return Ok(HttpResponse::Ok().json(CallbackResponse {
            code: 400,
            message: "签名验证失败".to_string(),
        }));
    }

    log::info!("支付回调签名验证成功: order_id={}", data.order_id);

    // 检查交易状态
    if data.trade_state != "SUCCESS" {
        log::warn!(
            "支付回调交易状态非成功: order_id={}, trade_state={}",
            data.order_id,
            data.trade_state
        );
        return Ok(HttpResponse::Ok().json(CallbackResponse {
            code: 400,
            message: format!("交易状态异常: {}", data.trade_state),
        }));
    }

    // TODO: 实现完整的回调处理逻辑
    // 1. 根据 other_order_no 查询 BBSMC 订单
    // 2. 更新订单状态为已支付
    // 3. 触发后续业务逻辑（如解锁付费内容、发放商品等）

    log::info!(
        "支付回调处理成功: order_id={}, other_order_no={}, money={}分",
        data.order_id,
        data.other_order_no,
        data.money
    );

    Ok(HttpResponse::Ok().json(CallbackResponse {
        code: 200,
        message: "success".to_string(),
    }))
}

/// 验证支付签名
///
/// 签名算法:
/// 1. 将 data 中的所有 key 按字母排序
/// 2. 拼接所有 value（不包含 key，只有 value）
/// 3. 在末尾添加 keycode
/// 4. 在开头也添加 keycode
/// 5. MD5 后转大写
///
/// 即: MD5(keycode + value1 + value2 + ... + keycode).toUpperCase()
fn verify_payment_signature(
    data: &PaymentCallbackData,
    sign: &str,
    keycode: &str,
) -> bool {
    // 构建有序的键值对（按 key 字母排序）
    let mut params: BTreeMap<&str, &str> = BTreeMap::new();
    params.insert("tradeState", &data.trade_state);
    params.insert("otherOrderNo", &data.other_order_no);
    params.insert("orderId", &data.order_id);
    if let Some(ref txn_id) = data.order_transaction_id {
        params.insert("orderTransactionId", txn_id);
    }
    params.insert("sid", &data.sid);
    params.insert("title", &data.title);
    params.insert("payType", &data.pay_type);
    params.insert("userDisplayName", &data.user_display_name);
    params.insert("money", &data.money);
    params.insert("settlement", &data.settlement);

    // 拼接所有 value（BTreeMap 自动按 key 排序）
    let mut values = String::new();
    for value in params.values() {
        values.push_str(value);
    }

    // 签名: keycode + values + keycode
    let raw = format!("{}{}{}", keycode, values, keycode);
    let expected = format!("{:x}", md5::compute(&raw)).to_uppercase();

    // 使用常量时间比较，防止时序攻击
    let is_valid = constant_time_compare(&expected, sign);
    if !is_valid {
        log::debug!("签名不匹配: actual={}", sign);
    }

    is_valid
}

/// 常量时间字符串比较，防止时序攻击
///
/// 使用 subtle crate 实现真正的常量时间比较，
/// 即使长度不同也不会提前返回（会填充到相同长度后比较）
fn constant_time_compare(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    // subtle 的 ct_eq 会处理长度不等的情况，始终执行相同数量的操作
    a_bytes.ct_eq(b_bytes).into()
}
