use super::DatabaseError;
use super::ids::*;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 购买状态
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PurchaseStatus {
    Active,   // 有效
    Expired,  // 已过期
    Refunded, // 已退款
}

impl PurchaseStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Expired => "expired",
            Self::Refunded => "refunded",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "active" => Self::Active,
            "expired" => Self::Expired,
            "refunded" => Self::Refunded,
            // 未知状态默认为 Expired，避免意外授权
            _ => {
                tracing::warn!("未知的购买状态: {}, 默认为 expired", s);
                Self::Expired
            }
        }
    }
}

/// 用户购买记录
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPurchase {
    pub id: UserPurchaseId,
    pub user_id: UserId,
    pub project_id: ProjectId,
    pub order_no: Option<String>,
    pub amount: Decimal,
    pub purchased_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: PurchaseStatus,
}

impl UserPurchase {
    /// 创建购买记录
    pub async fn create(
        user_id: UserId,
        project_id: ProjectId,
        order_no: Option<String>,
        amount: Decimal,
        expires_at: Option<DateTime<Utc>>,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Self, DatabaseError> {
        let new_id = generate_user_purchase_id(&mut *transaction).await?;
        let now = Utc::now();

        // 使用 RETURNING 获取实际的记录 ID 和 purchased_at
        // 当发生冲突（续费）时：
        // - id 和 purchased_at 保持原值（首次购买时间）
        // - amount 更新为本次支付金额（不累加，因为有订单表记录历史）
        // - expires_at 和 status 更新
        let result = sqlx::query!(
            "
            INSERT INTO user_purchases (id, user_id, project_id, order_no, amount, purchased_at, expires_at, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'active')
            ON CONFLICT (user_id, project_id) DO UPDATE SET
                order_no = COALESCE($4, user_purchases.order_no),
                amount = $5,
                expires_at = $7,
                status = 'active'
            RETURNING id, purchased_at
            ",
            new_id.0,
            user_id.0,
            project_id.0,
            order_no.as_deref(),
            amount,
            now,
            expires_at,
        )
        .fetch_one(&mut **transaction)
        .await?;

        Ok(Self {
            id: UserPurchaseId(result.id),
            user_id,
            project_id,
            order_no,
            amount,
            purchased_at: result.purchased_at,
            expires_at,
            status: PurchaseStatus::Active,
        })
    }

    /// 检查用户是否已购买项目（且未过期）
    pub async fn check_access<'a, E>(
        user_id: UserId,
        project_id: ProjectId,
        executor: E,
    ) -> Result<bool, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT id FROM user_purchases
            WHERE user_id = $1 AND project_id = $2 AND status = 'active'
              AND (expires_at IS NULL OR expires_at > NOW())
            ",
            user_id.0,
            project_id.0,
        )
        .fetch_optional(executor)
        .await?;

        Ok(result.is_some())
    }

    /// 获取用户对项目的购买记录
    pub async fn get<'a, E>(
        user_id: UserId,
        project_id: ProjectId,
        executor: E,
    ) -> Result<Option<Self>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT id, user_id, project_id, order_no, amount, purchased_at, expires_at, status
            FROM user_purchases
            WHERE user_id = $1 AND project_id = $2
            ",
            user_id.0,
            project_id.0,
        )
        .fetch_optional(executor)
        .await?;

        Ok(result.map(|row| Self {
            id: UserPurchaseId(row.id),
            user_id: UserId(row.user_id),
            project_id: ProjectId(row.project_id),
            order_no: row.order_no,
            amount: row.amount,
            purchased_at: row.purchased_at,
            expires_at: row.expires_at,
            status: PurchaseStatus::from_string(&row.status),
        }))
    }

    /// 获取用户的所有购买记录
    pub async fn get_user_purchases<'a, E>(
        user_id: UserId,
        executor: E,
    ) -> Result<Vec<Self>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let results = sqlx::query!(
            "
            SELECT id, user_id, project_id, order_no, amount, purchased_at, expires_at, status
            FROM user_purchases
            WHERE user_id = $1
            ORDER BY purchased_at DESC
            ",
            user_id.0,
        )
        .fetch_all(executor)
        .await?;

        Ok(results
            .into_iter()
            .map(|row| Self {
                id: UserPurchaseId(row.id),
                user_id: UserId(row.user_id),
                project_id: ProjectId(row.project_id),
                order_no: row.order_no,
                amount: row.amount,
                purchased_at: row.purchased_at,
                expires_at: row.expires_at,
                status: PurchaseStatus::from_string(&row.status),
            })
            .collect())
    }

    /// 获取项目的所有购买者
    pub async fn get_project_purchasers<'a, E>(
        project_id: ProjectId,
        executor: E,
    ) -> Result<Vec<Self>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let results = sqlx::query!(
            "
            SELECT id, user_id, project_id, order_no, amount, purchased_at, expires_at, status
            FROM user_purchases
            WHERE project_id = $1 AND status = 'active'
            ORDER BY purchased_at DESC
            ",
            project_id.0,
        )
        .fetch_all(executor)
        .await?;

        Ok(results
            .into_iter()
            .map(|row| Self {
                id: UserPurchaseId(row.id),
                user_id: UserId(row.user_id),
                project_id: ProjectId(row.project_id),
                order_no: row.order_no,
                amount: row.amount,
                purchased_at: row.purchased_at,
                expires_at: row.expires_at,
                status: PurchaseStatus::from_string(&row.status),
            })
            .collect())
    }

    /// 更新过期的购买记录状态
    pub async fn update_expired_purchases(
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<u64, DatabaseError> {
        let result = sqlx::query!(
            "
            UPDATE user_purchases
            SET status = 'expired'
            WHERE status = 'active' AND expires_at IS NOT NULL AND expires_at <= NOW()
            "
        )
        .execute(&mut **transaction)
        .await?;

        Ok(result.rows_affected())
    }
}
