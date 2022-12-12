// @generated automatically by Diesel CLI.

diesel::table! {
    twitter_accounts (id) {
        id -> Int8,
        name -> Text,
        username -> Text,
        created_at -> Timestamp,
        description -> Nullable<Text>,
        location -> Nullable<Text>,
        pinned_tweet_id -> Nullable<Int8>,
        profile_image_url -> Nullable<Text>,
        protected -> Bool,
        public_metrics -> Nullable<Jsonb>,
        url -> Nullable<Text>,
        verified -> Bool,
        withheld -> Nullable<Jsonb>,
    }
}

diesel::table! {
    twitter_credentials (name) {
        name -> Text,
        consumer_key -> Text,
        consumer_secret -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(twitter_accounts, twitter_credentials,);
