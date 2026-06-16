use sqlx::{
    Error, PgPool, Postgres,
    postgres::PgQueryResult,
    types::chrono::{DateTime, Utc},
};
use uuid::Uuid;

use crate::models::user::{Expense, UpdateExpense};

// CRUD operations
pub async fn create_expense(
    user_id: Uuid,
    amount: i32,
    currency: String,
    category: String,
    description: String,
    expense_data: DateTime<Utc>,
    pool: &PgPool,
) -> Result<PgQueryResult, Error> {
    sqlx::query("INSERT INTO expenses(user_id, amount, currency, category, description, expense_date ) VALUES($1, $2, $3, $4, $5, $6)")
        .bind(user_id)
        .bind(amount)
        .bind(currency)
        .bind(category)
        .bind(description)
        .bind(expense_data)
        .execute(pool)
        .await
}

pub async fn read_expense_by_user_id(user_id: Uuid, pool: &PgPool) -> Result<Vec<Expense>, Error> {
    sqlx::query_as::<_, Expense>("SELECT * FROM expenses WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn update_expense(
    id: i64,
    update_values: UpdateExpense,
    pool: &PgPool,
) -> Result<PgQueryResult, Error> {
    use sqlx::query_builder::QueryBuilder;

    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE expenses SET ");

    let mut first = true;

    if let Some(amount) = update_values.amount {
        if !first {
            qb.push(", ");
        }
        qb.push("amount = ").push_bind(amount);
        first = false;
    }
    if let Some(currency) = update_values.currency {
        if !first {
            qb.push(", ");
        }
        qb.push("currency = ").push_bind(currency);
        first = false;
    }
    if let Some(category) = update_values.category {
        if !first {
            qb.push(", ");
        }
        qb.push("category = ").push_bind(category);
        first = false;
    }
    if let Some(description) = update_values.description {
        if !first {
            qb.push(", ");
        }
        qb.push("description = ").push_bind(description);
        first = false;
    }
    if let Some(expense_date) = update_values.expense_date {
        if !first {
            qb.push(", ");
        }
        qb.push("expense_date = ").push_bind(expense_date);
    }

    qb.push(" WHERE id = ").push_bind(id);

    let query = qb.build();
    query.execute(pool).await
}

pub async fn delete_expense(id: i64, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query("DELETE FROM expenses WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
}
