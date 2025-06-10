use crate::todos::{CreateTodo, Todo, UpdateTodo};
use sqlx::postgres::PgPool;
use sqlx::{query, query_as, Execute, Postgres, QueryBuilder};
use std::env;
use std::env::VarError;

fn database_url() -> Result<String, VarError> {
    Ok(env::var("DATABASE_URL").expect("Unable to read DATBASE_URL"))
}

async fn get_pool() -> anyhow::Result<PgPool> {
    let url = database_url()?;
    Ok(PgPool::connect(&url).await?)
}

pub async fn todos() -> anyhow::Result<Vec<Todo>> {
    let pool = get_pool().await?;

    let records = query_as!(
        Todo,
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(&pool)
    .await?;

    Ok(records)
}

pub async fn create_todo(todo: CreateTodo) -> anyhow::Result<i64> {
    let pool = get_pool().await?;

    let record = query!(
        r#"
INSERT INTO todos ( description, done )
VALUES ( $1, $2 )
RETURNING id
  "#,
        todo.description,
        todo.done
    )
    .fetch_one(&pool)
    .await?;

    Ok(record.id)
}

pub async fn get_todo(id: i64) -> anyhow::Result<Todo> {
    let pool = get_pool().await?;

    let record = query_as!(
        Todo,
        r#"
SELECT id, description, done FROM todos
WHERE id = $1
  "#,
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(record)
}

pub async fn update_todo(id: i64, fields: UpdateTodo) -> anyhow::Result<bool> {
    let pool = get_pool().await?;

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE todos SET ");
    let mut separated = query_builder.separated(", ");

    if fields.done.is_some() {
        separated
            .push("done=")
            .push_bind_unseparated(fields.done.clone());
    }
    if fields.description.is_some() {
        separated
            .push("description=")
            .push_bind_unseparated(fields.description.clone());
    }
    query_builder.push(" WHERE id=").push_bind(id);

    let sql = query_builder.build().sql();
    let mut result = query(sql);

    if fields.done.is_some() {
        result = result.bind(fields.done);
    }
    if fields.description.is_some() {
        result = result.bind(fields.description);
    }

    let rows_affected = result.bind(id).execute(&pool).await?.rows_affected();

    Ok(rows_affected > 0)
}

pub async fn delete_todo(id: i64) -> anyhow::Result<bool> {
    let pool = get_pool().await?;

    let rows_affected = query!(r#"DELETE FROM todos WHERE id = $1"#, id)
        .execute(&pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}
