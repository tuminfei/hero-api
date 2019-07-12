table! {
    hero (id) {
        id -> Integer,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Integer,
        image_url -> Nullable<Text>,
    }
}

table! {
    login_history (id) {
        id -> Integer,
        user_id -> Integer,
        login_timestamp -> Timestamp,
    }
}

table! {
    people (id) {
        id -> Integer,
        first_name -> Varchar,
        last_name -> Varchar,
        age -> Integer,
        profession -> Varchar,
        salary -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    hero,
    login_history,
    people,
    users,
);
