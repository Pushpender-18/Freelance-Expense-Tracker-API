use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::types::BigDecimal;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::NaiveDate;
use sqlx::types::chrono::Utc;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)] // Remove Debug
pub struct Expense {
    pub id: i64,
    pub user_id: Uuid,
    pub amount: BigDecimal,
    pub currency: String,
    pub category: String,
    pub description: String,
    pub expense_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct UpdateExpense {
    pub amount: Option<BigDecimal>,
    pub currency: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub expense_date: Option<NaiveDate>,
}

pub struct RefreshTokens {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: i64,
}

impl Claims {
    pub fn new(id: Uuid, exp: i64) -> Claims {
        Claims {
            sub: id.to_string(),
            exp,
        }
    }
}
