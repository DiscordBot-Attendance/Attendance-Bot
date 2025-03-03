use actix_web::{web, HttpResponse, Responder};

use crate::{api::application::member_service::show_members, config::database::DBPool};

pub async fn show_member_handler(
    pool: web::Data<DBPool>,
    path: web::Path<String>,
) -> impl Responder {
    let team_name = path.into_inner();

    match web::block(move || show_members(&pool, &team_name)).await {
        Ok(Ok(members)) => HttpResponse::Ok().json(members),
        Ok(Err(e)) => HttpResponse::InternalServerError().body(e),
        Err(e) => HttpResponse::InternalServerError().body(format!("Threading error: {}", e)),
    }
}
