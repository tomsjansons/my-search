use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(rename = "incomplete_results")]
    pub incomplete_results: bool,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub url: String,
    // #[serde(rename = "repository_url")]
    // pub repository_url: String,
    // #[serde(rename = "labels_url")]
    // pub labels_url: String,
    // #[serde(rename = "comments_url")]
    // pub comments_url: String,
    // #[serde(rename = "events_url")]
    // pub events_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub id: i64,
    // #[serde(rename = "node_id")]
    // pub node_id: String,
    // pub number: i64,
    pub title: String,
    // pub user: User,
    // pub labels: Vec<Label>,
    // pub state: String,
    // pub locked: bool,
    // pub assignee: Option<Assignee>,
    // pub assignees: Vec<Assignee2>,
    // // pub milestone: Value,
    // pub comments: i64,
    // #[serde(rename = "created_at")]
    // pub created_at: String,
    // #[serde(rename = "updated_at")]
    // pub updated_at: String,
    // // #[serde(rename = "closed_at")]
    // // pub closed_at: Value,
    // #[serde(rename = "author_association")]
    // pub author_association: String,
    // // #[serde(rename = "type")]
    // // pub type_field: Value,
    // // #[serde(rename = "active_lock_reason")]
    // // pub active_lock_reason: Value,
    // pub draft: bool,
    // #[serde(rename = "pull_request")]
    // pub pull_request: PullRequest,
    // pub body: Option<String>,
    // pub reactions: Reactions,
    // #[serde(rename = "timeline_url")]
    // pub timeline_url: String,
    // // #[serde(rename = "performed_via_github_app")]
    // // pub performed_via_github_app: Value,
    // // #[serde(rename = "state_reason")]
    // // pub state_reason: Value,
    // pub score: f64,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct User {
//     pub login: String,
//     pub id: i64,
//     #[serde(rename = "node_id")]
//     pub node_id: String,
//     #[serde(rename = "avatar_url")]
//     pub avatar_url: String,
//     #[serde(rename = "gravatar_id")]
//     pub gravatar_id: String,
//     pub url: String,
//     #[serde(rename = "html_url")]
//     pub html_url: String,
//     #[serde(rename = "followers_url")]
//     pub followers_url: String,
//     #[serde(rename = "following_url")]
//     pub following_url: String,
//     #[serde(rename = "gists_url")]
//     pub gists_url: String,
//     #[serde(rename = "starred_url")]
//     pub starred_url: String,
//     #[serde(rename = "subscriptions_url")]
//     pub subscriptions_url: String,
//     #[serde(rename = "organizations_url")]
//     pub organizations_url: String,
//     #[serde(rename = "repos_url")]
//     pub repos_url: String,
//     #[serde(rename = "events_url")]
//     pub events_url: String,
//     #[serde(rename = "received_events_url")]
//     pub received_events_url: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     #[serde(rename = "user_view_type")]
//     pub user_view_type: String,
//     #[serde(rename = "site_admin")]
//     pub site_admin: bool,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Label {
//     pub id: i64,
//     #[serde(rename = "node_id")]
//     pub node_id: String,
//     pub url: String,
//     pub name: String,
//     pub color: String,
//     pub default: bool,
//     pub description: Option<String>,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Assignee {
//     pub login: String,
//     pub id: i64,
//     #[serde(rename = "node_id")]
//     pub node_id: String,
//     #[serde(rename = "avatar_url")]
//     pub avatar_url: String,
//     #[serde(rename = "gravatar_id")]
//     pub gravatar_id: String,
//     pub url: String,
//     #[serde(rename = "html_url")]
//     pub html_url: String,
//     #[serde(rename = "followers_url")]
//     pub followers_url: String,
//     #[serde(rename = "following_url")]
//     pub following_url: String,
//     #[serde(rename = "gists_url")]
//     pub gists_url: String,
//     #[serde(rename = "starred_url")]
//     pub starred_url: String,
//     #[serde(rename = "subscriptions_url")]
//     pub subscriptions_url: String,
//     #[serde(rename = "organizations_url")]
//     pub organizations_url: String,
//     #[serde(rename = "repos_url")]
//     pub repos_url: String,
//     #[serde(rename = "events_url")]
//     pub events_url: String,
//     #[serde(rename = "received_events_url")]
//     pub received_events_url: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     #[serde(rename = "user_view_type")]
//     pub user_view_type: String,
//     #[serde(rename = "site_admin")]
//     pub site_admin: bool,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Assignee2 {
//     pub login: String,
//     pub id: i64,
//     #[serde(rename = "node_id")]
//     pub node_id: String,
//     #[serde(rename = "avatar_url")]
//     pub avatar_url: String,
//     #[serde(rename = "gravatar_id")]
//     pub gravatar_id: String,
//     pub url: String,
//     #[serde(rename = "html_url")]
//     pub html_url: String,
//     #[serde(rename = "followers_url")]
//     pub followers_url: String,
//     #[serde(rename = "following_url")]
//     pub following_url: String,
//     #[serde(rename = "gists_url")]
//     pub gists_url: String,
//     #[serde(rename = "starred_url")]
//     pub starred_url: String,
//     #[serde(rename = "subscriptions_url")]
//     pub subscriptions_url: String,
//     #[serde(rename = "organizations_url")]
//     pub organizations_url: String,
//     #[serde(rename = "repos_url")]
//     pub repos_url: String,
//     #[serde(rename = "events_url")]
//     pub events_url: String,
//     #[serde(rename = "received_events_url")]
//     pub received_events_url: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     #[serde(rename = "user_view_type")]
//     pub user_view_type: String,
//     #[serde(rename = "site_admin")]
//     pub site_admin: bool,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PullRequest {
//     pub url: String,
//     #[serde(rename = "html_url")]
//     pub html_url: String,
//     #[serde(rename = "diff_url")]
//     pub diff_url: String,
//     #[serde(rename = "patch_url")]
//     pub patch_url: String,
//     // #[serde(rename = "merged_at")]
//     // pub merged_at: Value,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Reactions {
//     pub url: String,
//     #[serde(rename = "total_count")]
//     pub total_count: i64,
//     #[serde(rename = "+1")]
//     pub n1: i64,
//     #[serde(rename = "-1")]
//     pub n12: i64,
//     pub laugh: i64,
//     pub hooray: i64,
//     pub confused: i64,
//     pub heart: i64,
//     pub rocket: i64,
//     pub eyes: i64,
// }
