use actix_web::{web, HttpResponse, Responder};

use crate::{
    api::application::attendance_service::show_member_attendance, config::database::DBPool,
};

pub async fn show_member_attendance_handler(
    pool: web::Data<DBPool>,
    path: web::Path<String>,
) -> impl Responder {
    let team_name = path.into_inner();

    match web::block(move || show_member_attendance(&pool, &team_name)).await {
        Ok(Ok(attendance)) => HttpResponse::Ok().json(attendance),
        Ok(Err(e)) => HttpResponse::InternalServerError().body(e),
        Err(e) => HttpResponse::InternalServerError().body(format!("Threading error: {}", e)),
    }
}
