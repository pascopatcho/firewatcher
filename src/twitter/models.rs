use crate::schema::{twitter_accounts, twitter_credentials};
use chrono::prelude::*;
use diesel::prelude::*;
use serde_json::Value as JsonValue;

#[derive(Queryable)]
pub struct TwitterAccount {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub description: Option<String>,
    pub location: Option<String>,
    pub pinned_tweet_id: Option<String>,
    pub profile_image_url: Option<String>,
    pub protected: bool,
    pub public_metrics: Option<JsonValue>,
    pub url: Option<String>,
    pub verified: bool,
    pub withheld: Option<JsonValue>,
}

#[derive(Insertable)]
#[diesel(table_name = twitter_accounts)]
pub struct NewTwitterAccount<'a> {
    pub id: i64,
    pub name: &'a str,
    pub username: &'a str,
    pub created_at: &'a NaiveDateTime,
    pub description: Option<&'a str>,
    pub location: Option<&'a str>,
    pub pinned_tweet_id: Option<i64>,
    pub profile_image_url: Option<&'a str>,
    pub protected: bool,
    pub public_metrics: Option<&'a JsonValue>,
    pub url: Option<&'a str>,
    pub verified: bool,
    pub withheld: Option<&'a JsonValue>,
}

#[derive(Queryable)]
pub struct TwitterCredential {
    pub name: String,
    pub consumer_key: String,
    pub consumer_secret: String,
}

#[derive(Insertable)]
#[diesel(table_name = twitter_credentials)]
pub struct NewTwitterCredential<'a> {
    pub name: &'a str,
    pub consumer_key: &'a str,
    pub consumer_secret: &'a str,
}
