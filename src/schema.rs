table! {
    artists (id) {
        id -> Uuid,
        name -> Text,
    }
}

table! {
    event_artists (id) {
        id -> Uuid,
        event_id -> Uuid,
        artist_id -> Uuid,
        rank -> Int4,
    }
}

table! {
    event_histories (id) {
        id -> Uuid,
        event_id -> Uuid,
        order_id -> Uuid,
        user_id -> Uuid,
        protocol_reference_hash -> Varchar,
    }
}

table! {
    events (id) {
        id -> Uuid,
        organization_id -> Uuid,
        venue_id -> Uuid,
    }
}

table! {
    orders (id) {
        id -> Uuid,
        user_id -> Uuid,
        event_id -> Uuid,
    }
}

table! {
    organizations (id) {
        id -> Uuid,
        owner_user_id -> Uuid,
        name -> Text,
        address -> Nullable<Text>,
        city -> Nullable<Text>,
        state -> Nullable<Text>,
        country -> Nullable<Text>,
        zip -> Nullable<Text>,
        phone -> Nullable<Text>,
    }
}

table! {
    organization_users (id) {
        id -> Uuid,
        organization_id -> Uuid,
        user_id -> Uuid,
    }
}

table! {
    organization_venues (id) {
        id -> Uuid,
        organization_id -> Uuid,
        venue_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        phone -> Text,
        hashed_pw -> Text,
        created_at -> Timestamp,
        last_used -> Nullable<Timestamp>,
        active -> Bool,
        role -> Int4,
    }
}

table! {
    venues (id) {
        id -> Uuid,
        name -> Text,
    }
}

joinable!(event_artists -> artists (artist_id));
joinable!(event_artists -> events (event_id));
joinable!(event_histories -> events (event_id));
joinable!(event_histories -> orders (order_id));
joinable!(event_histories -> users (user_id));
joinable!(events -> organizations (organization_id));
joinable!(events -> venues (venue_id));
joinable!(orders -> events (event_id));
joinable!(orders -> users (user_id));
joinable!(organization_users -> organizations (organization_id));
joinable!(organization_users -> users (user_id));
joinable!(organization_venues -> organizations (organization_id));
joinable!(organization_venues -> venues (venue_id));
joinable!(organizations -> users (owner_user_id));

allow_tables_to_appear_in_same_query!(
    artists,
    event_artists,
    event_histories,
    events,
    orders,
    organizations,
    organization_users,
    organization_venues,
    users,
    venues,
);
