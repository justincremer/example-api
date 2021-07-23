table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        created_on -> Timestamp,
        updated_on -> Timestamp,
        body -> Varchar,
        up_votes -> Int4,
        down_votes -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        created_on -> Timestamp,
        updated_on -> Timestamp,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        password -> Varchar,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(posts, users,);
