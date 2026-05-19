use crate::auth::get_user_from_headers;
use crate::database::redis::RedisPool;
use crate::models::ids::PayoutId;
use crate::models::pats::Scopes;
use crate::queue::payouts::{PayoutsQueue, make_aditude_request};
use crate::queue::session::AuthQueue;
use crate::routes::ApiError;
use actix_web::{HttpRequest, HttpResponse, delete, get, post, web};
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("payout")
            .service(user_payouts)
            .service(create_payout)
            .service(cancel_payout)
            .service(payment_methods)
            .service(get_balance)
            .service(platform_revenue),
    );
}

#[get("")]
pub async fn user_payouts(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
    session_queue: web::Data<AuthQueue>,
) -> Result<HttpResponse, ApiError> {
    let user = get_user_from_headers(
        &req,
        &**pool,
        &redis,
        &session_queue,
        Some(&[Scopes::PAYOUTS_READ]),
    )
    .await?
    .1;

    let payout_ids =
        crate::database::models::payout_item::Payout::get_all_for_user(
            user.id.into(),
            &**pool,
        )
        .await?;
    let payouts = crate::database::models::payout_item::Payout::get_many(
        &payout_ids,
        &**pool,
    )
    .await?;

    Ok(HttpResponse::Ok().json(
        payouts
            .into_iter()
            .map(crate::models::payouts::Payout::from)
            .collect::<Vec<_>>(),
    ))
}

#[post("")]
pub async fn create_payout(
    _req: HttpRequest,
    _pool: web::Data<PgPool>,
    _redis: web::Data<RedisPool>,
    _session_queue: web::Data<AuthQueue>,
    _payouts_queue: web::Data<PayoutsQueue>,
) -> Result<HttpResponse, ApiError> {
    // 提现通道升级中：旧的 PayPal/Tremendous/Venmo 已下线，新通道（云账户）尚未接入
    Err(ApiError::InvalidInput(
        "提现功能升级中，暂未开放，请稍后再试。".to_string(),
    ))
}

#[delete("{id}")]
pub async fn cancel_payout(
    info: web::Path<(PayoutId,)>,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
    _payouts_queue: web::Data<PayoutsQueue>,
    session_queue: web::Data<AuthQueue>,
) -> Result<HttpResponse, ApiError> {
    let _user = get_user_from_headers(
        &req,
        &**pool,
        &redis,
        &session_queue,
        Some(&[Scopes::PAYOUTS_WRITE]),
    )
    .await?
    .1;

    let _ = info.into_inner().0;
    // 提现通道下线后无法发起新的取消请求；遗留待处理记录请由管理员手动结算
    Err(ApiError::InvalidInput(
        "提现功能升级中，暂未开放取消操作。".to_string(),
    ))
}

#[derive(Deserialize)]
pub struct MethodFilter {
    pub country: Option<String>,
}

#[get("methods")]
pub async fn payment_methods(
    _payouts_queue: web::Data<PayoutsQueue>,
    _filter: web::Query<MethodFilter>,
) -> Result<HttpResponse, ApiError> {
    // 通道升级中：返回空列表，前端据此显示"暂未开放"提示
    Ok(HttpResponse::Ok().json(Vec::<serde_json::Value>::new()))
}

#[derive(Serialize)]
pub struct UserBalance {
    pub available: Decimal,
    pub pending: Decimal,
}

#[get("balance")]
pub async fn get_balance(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
    session_queue: web::Data<AuthQueue>,
) -> Result<HttpResponse, ApiError> {
    let user = get_user_from_headers(
        &req,
        &**pool,
        &redis,
        &session_queue,
        Some(&[Scopes::PAYOUTS_READ]),
    )
    .await?
    .1;

    let balance = get_user_balance(user.id.into(), &pool).await?;

    Ok(HttpResponse::Ok().json(balance))
}

async fn get_user_balance(
    user_id: crate::database::models::ids::UserId,
    pool: &PgPool,
) -> Result<UserBalance, sqlx::Error> {
    let available = sqlx::query!(
        "
        SELECT SUM(amount)
        FROM payouts_values
        WHERE user_id = $1 AND date_available <= NOW()
        ",
        user_id.0
    )
    .fetch_optional(pool)
    .await?;

    let pending = sqlx::query!(
        "
        SELECT SUM(amount)
        FROM payouts_values
        WHERE user_id = $1 AND date_available > NOW()
        ",
        user_id.0
    )
    .fetch_optional(pool)
    .await?;

    let withdrawn = sqlx::query!(
        "
        SELECT SUM(amount) amount, SUM(fee) fee
        FROM payouts
        WHERE user_id = $1 AND (status = 'success' OR status = 'in-transit')
        ",
        user_id.0
    )
    .fetch_optional(pool)
    .await?;

    let available = available
        .map(|x| x.sum.unwrap_or(Decimal::ZERO))
        .unwrap_or(Decimal::ZERO);
    let pending = pending
        .map(|x| x.sum.unwrap_or(Decimal::ZERO))
        .unwrap_or(Decimal::ZERO);
    let (withdrawn, fees) = withdrawn
        .map(|x| {
            (
                x.amount.unwrap_or(Decimal::ZERO),
                x.fee.unwrap_or(Decimal::ZERO),
            )
        })
        .unwrap_or((Decimal::ZERO, Decimal::ZERO));

    Ok(UserBalance {
        available: available.round_dp(16)
            - withdrawn.round_dp(16)
            - fees.round_dp(16),
        pending,
    })
}

#[derive(Serialize, Deserialize)]
pub struct RevenueResponse {
    pub all_time: Decimal,
    pub data: Vec<RevenueData>,
}

#[derive(Serialize, Deserialize)]
pub struct RevenueData {
    pub time: u64,
    pub revenue: Decimal,
    pub creator_revenue: Decimal,
}

#[get("platform_revenue")]
pub async fn platform_revenue(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
) -> Result<HttpResponse, ApiError> {
    let mut redis = redis.connect().await?;

    const PLATFORM_REVENUE_NAMESPACE: &str = "platform_revenue";

    let res: Option<RevenueResponse> = redis
        .get_deserialized_from_json(PLATFORM_REVENUE_NAMESPACE, "0")
        .await?;

    if let Some(res) = res {
        return Ok(HttpResponse::Ok().json(res));
    }

    let all_time_payouts = sqlx::query!(
        "
        SELECT SUM(amount) from payouts_values
        ",
    )
    .fetch_optional(&**pool)
    .await?
    .and_then(|x| x.sum)
    .unwrap_or(Decimal::ZERO);

    let points = make_aditude_request(
        &["METRIC_REVENUE", "METRIC_IMPRESSIONS"],
        "30d",
        "1d",
    )
    .await?;

    let mut points_map = HashMap::new();

    for point in points {
        for point in point.points_list {
            let entry =
                points_map.entry(point.time.seconds).or_insert((None, None));

            if let Some(revenue) = point.metric.revenue {
                entry.0 = Some(revenue);
            }

            if let Some(impressions) = point.metric.impressions {
                entry.1 = Some(impressions);
            }
        }
    }

    let mut revenue_data = Vec::new();
    let now = Utc::now();

    for i in 1..=30 {
        let time = now - Duration::days(i);
        let start = time
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp();

        if let Some((revenue, impressions)) = points_map.remove(&(start as u64))
        {
            // 2024/9/5 之前，旧版提现机制生效期间
            if start >= 1725494400 {
                let revenue = revenue.unwrap_or(Decimal::ZERO);
                let impressions = impressions.unwrap_or(0);

                // BBSMC 的广告收入分成
                let platform_cut = Decimal::from(1) / Decimal::from(4);
                // Clean.io 费用（广告反恶意软件），按每千次展示计算
                let clean_io_fee = Decimal::from(8) / Decimal::from(1000);

                let net_revenue = revenue
                    - (clean_io_fee * Decimal::from(impressions)
                        / Decimal::from(1000));

                let payout = net_revenue * (Decimal::from(1) - platform_cut);

                revenue_data.push(RevenueData {
                    time: start as u64,
                    revenue: net_revenue,
                    creator_revenue: payout,
                });

                continue;
            }
        }

        revenue_data.push(get_legacy_data_point(start as u64));
    }

    let res = RevenueResponse {
        all_time: all_time_payouts,
        data: revenue_data,
    };

    redis
        .set_serialized_to_json(
            PLATFORM_REVENUE_NAMESPACE,
            0,
            &res,
            Some(60 * 60),
        )
        .await?;

    Ok(HttpResponse::Ok().json(res))
}

fn get_legacy_data_point(timestamp: u64) -> RevenueData {
    let start = Utc.timestamp_opt(timestamp as i64, 0).unwrap();

    let old_payouts_budget = Decimal::from(10_000);

    let days = Decimal::from(28);
    let weekdays = Decimal::from(20);
    let weekend_bonus = Decimal::from(5) / Decimal::from(4);

    let weekday_amount =
        old_payouts_budget / (weekdays + (weekend_bonus) * (days - weekdays));
    let weekend_amount = weekday_amount * weekend_bonus;

    let payout = match start.weekday() {
        Weekday::Sat | Weekday::Sun => weekend_amount,
        _ => weekday_amount,
    };

    RevenueData {
        time: timestamp,
        revenue: payout,
        creator_revenue: payout * (Decimal::from(9) / Decimal::from(10)),
    }
}
