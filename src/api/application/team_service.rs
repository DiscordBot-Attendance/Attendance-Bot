use crate::{
    bot::{application::services::team_service, domain::table::TeamTable},
    config::database::DBPool,
};

pub fn show_teams(pool: &DBPool, admin_discord_id: &str) -> Result<Vec<TeamTable>, String> {
    let mut conn = pool.get().expect("Failed to connect to database!");
    team_service::show_team(&mut conn, admin_discord_id)
}
