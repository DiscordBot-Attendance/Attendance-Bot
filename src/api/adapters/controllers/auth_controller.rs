use crate::api::domain::dto::AuthRequest;
use crate::{
    api::{application::auth_service::login_user, domain::dto::AuthResponse},
    config::database::DBPool,
};
use actix_web::{web, HttpResponse, Responder};

pub async fn login(pool: web::Data<DBPool>, form: web::Json<AuthRequest>) -> impl Responder {
    match login_user(&pool, &form.username, &form.password) {
        Ok(token) => HttpResponse::Ok().json(AuthResponse { token }),
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}
