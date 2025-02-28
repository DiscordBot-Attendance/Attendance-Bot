use crate::{api::domain::model::User, config::database::DBPool};
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use diesel::{ExpressionMethods, QueryDsl};

pub fn find_by_username(pool: &DBPool, username: &str) -> Option<User> {
    use crate::schema::users::dsl::{username as usersname, users};

    let mut conn = pool.get().expect("Failed to get DB connection!");

    users
        .filter(usersname.eq(username))
        .select(User::as_select())
        .first(&mut conn)
        .ok()
}
