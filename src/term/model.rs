use crate::intl::Lang;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use chrono::prelude::*;
use futures::future::{ready, Ready};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    id: i32,
    phrase: String,
    lang: String,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

impl Responder for Term {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok().json(body)))
    }
}

impl Term {
    pub async fn find_by_id(_lang: Lang, id: i32, pool: &PgPool) -> Result<Term> {
        let rec = sqlx::query_as!(
            Term,
            "
            SELECT *
            FROM terms
            WHERE id = $1
            ",
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(rec)
    }
}
