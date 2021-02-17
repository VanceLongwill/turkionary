use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};

pub enum Source {
    Tureng,
    Tdk,
}

#[derive(Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    id: i32,
    lang: String,
    source: Source,
    content: String,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

impl Responder for Definition {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok().json(body)))
    }
}

impl Definition {
    pub async fn find_by_term_id(
        _lang: Lang,
        term_id: i32,
        pool: &PgPool,
    ) -> Result<Vec<Definition>> {
        let rec = sqlx::query_as!(
            Definition,
            "
            SELECT *
            FROM definitions
            WHERE term_id = ?
            ",
            term_id
        )
        .fetch(pool)
        .await?;

        Ok(rec)
    }
}
