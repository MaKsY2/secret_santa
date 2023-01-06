// @generated automatically by Diesel CLI.

diesel::table! {
    groups (group_id) {
        group_id -> Int4,
        name -> Varchar,
        status -> Varchar,
    }
}

diesel::table! {
    memberships (user_id, group_id) {
        group_id -> Int4,
        user_id -> Int4,
        role -> Varchar,
    }
}

diesel::table! {
    santas (group_id, santa_user_id) {
        group_id -> Int4,
        santa_user_id -> Int4,
        reciever_user_id -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(memberships -> groups (group_id));
diesel::joinable!(memberships -> users (user_id));
diesel::joinable!(santas -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    memberships,
    santas,
    users,
);
