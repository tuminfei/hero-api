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
    people (id) {
        id -> Integer,
        first_name -> Varchar,
        last_name -> Varchar,
        age -> Integer,
        profession -> Varchar,
        salary -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    hero,
    people,
);
