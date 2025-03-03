use actix_web::{web, HttpResponse, Responder};

use crate::{api::application::team_service::show_teams, config::database::DBPool};

pub async fn show_teams_handler(
    pool: web::Data<DBPool>,
    path: web::Path<String>,
) -> impl Responder {
    let admin_discord_id = path.into_inner();

    match show_teams(pool.as_ref().clone(), &admin_discord_id) {
        Ok(teams) => HttpResponse::Ok().json(teams),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
