// @generated automatically by Diesel CLI.

diesel::table! {
    member_attendance (id) {
        id -> Int4,
        team_id -> Nullable<Int4>,
        member_id -> Nullable<Int4>,
        date -> Date,
        check_in_time -> Nullable<Timestamp>,
        check_out_time -> Nullable<Timestamp>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        team_id -> Nullable<Int4>,
        #[max_length = 50]
        discord_id -> Varchar,
        #[max_length = 50]
        position -> Nullable<Varchar>,
        join_date -> Nullable<Date>,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        admin_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 50]
        discord_id -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        is_admin -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(member_attendance -> members (member_id));
diesel::joinable!(member_attendance -> teams (team_id));
diesel::joinable!(members -> teams (team_id));
diesel::joinable!(teams -> users (admin_id));

diesel::allow_tables_to_appear_in_same_query!(
    member_attendance,
    members,
    teams,
    users,
);
