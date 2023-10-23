use sqlx::{types::time::PrimitiveDateTime, FromRow, Pool, Postgres};

#[derive(FromRow, Debug)]
pub struct Movie {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
}

pub enum Status {
    PREPARE,
    ON,
    OFF,
}

impl Status {
    pub fn to_code(&self) -> String {
        match *self {
            Status::PREPARE => "0".to_string(),
            Status::ON => "1".to_string(),
            Status::OFF => "99".to_string(),
        }
    }

    pub fn to_status(&self) -> String {
        match *self {
            Status::PREPARE => "未上映".to_string(),
            Status::ON => "上映".to_string(),
            Status::OFF => "下檔".to_string(),
        }
    }

    pub fn to_status_string(code: String) -> String {
        match code.as_str() {
            "0" => "未上映".to_string(),
            "1" => "上映".to_string(),
            "99" => "下檔".to_string(),
            _ => code,
        }
    }
}

pub async fn find_all(pool: Pool<Postgres>) -> Vec<Movie> {
    return sqlx::query_as!(
        Movie,
        r#"
            SELECT *
              FROM movie
             ORDER BY id
        "#
    )
    .fetch_all(&pool)
    .await
    .expect("find_all sql error");
}

pub async fn find_by_name(pool: Pool<Postgres>, name: String) -> Vec<Movie> {
    return sqlx::query_as!(
        Movie,
        r#"
            SELECT *
              FROM movie
             WHERE name like $1   
             ORDER BY id
        "#
    ,name + "%")
    .fetch_all(&pool)
    .await
    .expect("find_all sql error");
}

pub async fn insert(pool: Pool<Postgres>, movie: Movie) {
    sqlx::query!(
        r#"
            INSERT INTO movie (
                name, 
                status, 
                description, 
                created_at 
            ) VALUES (
                $1, 
                $2, 
                $3, 
                $4
            );
        "#,
        movie.name.unwrap(),
        movie.status.unwrap(),
        movie.description.unwrap(),
        movie.created_at.unwrap(),
    )
    .execute(&pool)
    .await
    .expect("insert sql error");
}

pub async fn update(pool: Pool<Postgres>, movie: Movie) {
    sqlx::query!(
        r#"
            UPDATE movie
            SET 
                name = $1,
                status = $2,
                description = $3,
                updated_at = $4
            WHERE 
                id = $5;
        "#,
        movie.name.unwrap(),
        movie.status.unwrap(),
        movie.description.unwrap(),
        movie.updated_at.unwrap(),
        movie.id.unwrap()
    )
    .execute(&pool)
    .await
    .expect("update sql error");
}

pub async fn find_by_id(pool: Pool<Postgres>, id: i32) -> Option<Movie> {
    return sqlx::query_as!(
        Movie,
        r#"
            SELECT *
              FROM movie
             WHERE id = $1
             ORDER BY id
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .expect("find_by_id sql error");
}

pub async fn delete(pool: Pool<Postgres>, id: i32) -> u64 {
    return sqlx::query!(r#"delete from  movie where id = $1"#, id)
        .execute(&pool)
        .await
        .expect("delete sql error")
        .rows_affected();
}
