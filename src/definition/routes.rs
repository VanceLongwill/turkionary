use crate::intl::Lang;
use crate::term::Term;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/definition/{from}/{to}/{id}")]
async fn find(
    from: web::Path<Lang>,
    to: web::Path<Lang>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = Term::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(term) => HttpResponse::Ok().json(term),
        _ => HttpResponse::NotFound().body("Term not found"),
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
}
