// @generated automatically by Diesel CLI.

diesel::table! {
    groups (group_id) {
        group_id -> Int4,
        status -> Varchar,
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
    user_to_group (user_id, group_id) {
        group_id -> Int4,
        user_id -> Int4,
        role -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(santas -> groups (group_id));
diesel::joinable!(user_to_group -> groups (group_id));
diesel::joinable!(user_to_group -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    santas,
    user_to_group,
    users,
);
