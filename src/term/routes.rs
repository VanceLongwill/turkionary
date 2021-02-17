use crate::intl::Lang;
use crate::term::Term;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/term/{lang}/{id}")]
async fn find(
    lang: web::Path<Lang>,
    id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    /// @TODO: use lang
    let result = Term::find_by_id(lang.into_inner(), id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(term) => HttpResponse::Ok().json(term),
        _ => HttpResponse::NotFound().body("Term not found"),
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
}
