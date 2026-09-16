// @generated automatically by Diesel CLI.

diesel::table! {
    experiences (id) {
        id -> Uuid,
        practice_id -> Uuid,
        user_id -> Uuid,
        note -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    listing_requests (id) {
        id -> Uuid,
        source_url -> Text,
        user_id -> Uuid,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    practice_sources (practice_id, source_id) {
        practice_id -> Uuid,
        source_id -> Uuid,
    }
}

diesel::table! {
    practices (id) {
        id -> Uuid,
        title -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    sources (id) {
        id -> Uuid,
        url -> Text,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(experiences -> practices (practice_id));
diesel::joinable!(practice_sources -> practices (practice_id));
diesel::joinable!(practice_sources -> sources (source_id));

diesel::allow_tables_to_appear_in_same_query!(
    experiences,
    listing_requests,
    practice_sources,
    practices,
    sources,
);
