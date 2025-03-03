use crate::{
    bot::{application::services::team_service::get_members_by_team, domain::table::MemberTable},
    config::database::DBPool,
};

pub fn show_members(pool: &DBPool, team_name: &str) -> Result<Vec<MemberTable>, String> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    get_members_by_team(&mut conn, team_name)
}
