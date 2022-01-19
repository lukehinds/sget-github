use crate::error::ReqError;
use chrono::offset;
use std::env;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use url::Url;

// import ureq Response struct

/* 
The following structs :
PullRequest, User, Label, Milestone, RequestedTeam, Permissions, License, Head, Base, Links, SelfLink, HtmlLink, IssueLink, CommentsLink, ReviewCommentsLink, ReviewCommentLink, CommitsLink, StatusesLink, PullRequestLink
were taken and slightly modified from 
https://github.com/XAMPPRocky/octocrab/blob/master/src/models/pulls.rs and
https://github.com/XAMPPRocky/octocrab/blob/master/src/models.rs
*/

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequest {
    pub url: String,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comments_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comment_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses_url: Option<Url>,
    pub number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub maintainer_can_modify: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Box<Milestone>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Box<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_reviewers: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_teams: Option<Vec<RequestedTeam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub head: Box<Head>,
    pub base: Box<Base>,
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<Links>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_association: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Box<Repository>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct User {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: Url,
    pub gravatar_id: String,
    pub url: Url,
    pub html_url: Url,
    pub followers_url: Url,
    pub following_url: Url,
    pub gists_url: Url,
    pub starred_url: Url,
    pub subscriptions_url: Url,
    pub organizations_url: Url,
    pub repos_url: Url,
    pub events_url: Url,
    pub received_events_url: Url,
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Label {
    pub id: i64,
    pub node_id: String,
    pub url: Url,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Milestone {
    pub url: Url,
    pub html_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_url: Option<Url>,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_issues: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_issues: Option<i64>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequestedTeam {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub html_url: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub privacy: String,
    pub permission: String,
    pub members_url: String,
    pub repositories_url: String
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Repository {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork: Option<bool>,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blobs_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborators_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloads_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forks_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commits_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_refs_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_tags_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_comment_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_events_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merges_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestones_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulls_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub releases_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stargazers_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trees_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svn_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<::serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forks_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stargazers_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchers_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_issues_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<Box<Repository>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Permissions {
    #[serde(default)]
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
    #[serde(default)]
    pub triage: bool,
    #[serde(default)]
    pub maintain: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct License {
    pub key: String,
    pub name: String,
    pub node_id: String,
    pub spdx_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    pub html_url: Option<Url>,
    pub description: Option<String>,
    pub implementation: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub conditions: Option<Vec<String>>,
    pub limitations: Option<Vec<String>>,
    pub body: Option<String>,
    pub featured: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Head {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Base {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Links {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<SelfLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_link: Option<HtmlLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_link: Option<IssueLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_link: Option<CommentsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comments_link: Option<ReviewCommentsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comment_link: Option<ReviewCommentLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_link: Option<CommitsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses_link: Option<StatusesLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pull_request")]
    pub pull_request_link: Option<PullRequestLink>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HtmlLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommentsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReviewCommentsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReviewCommentLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusesLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestLink {
    pub href: Url,
}
#[derive(Serialize, Deserialize, Debug)]
struct Content {
    #[serde(rename = "ref")]
    ref_: String,
    node_id: String,
    url: String,
    object: Object,
}

#[derive(Serialize, Deserialize, Debug)]
struct Object {
    sha: String,
    #[serde(rename = "type")]
    type_: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TreeEntry {
    sha: String,
    url: String,
    tree: Vec<TreeObject>,
    truncated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TreeObject {
    path: String,
    mode: String,
    #[serde(rename = "type")]
    type_: String,
    size: Option<u64>,
    sha: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Commit {
    sha: String,
    node_id: String,
    url: String,
    html_url: String,
    author: Author,
    committer: Author,
    message: String,
    tree: Tree,
    parents: Vec<Parent>,
    verification: Verification,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
    email: String,
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tree {
    sha: String,
    url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Parent {
    pub sha: String,
    url: String,
    html_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Verification {
    verified: bool,
    reason: String,
    signature: Option<String>,
    payload: Option<String>,
}

fn get_github_token() -> String {
    match env::var("GITHUB_AUTH_TOKEN") {
        Ok(val) => format!("Token {}", val),
        Err(_err) => ("").to_string(),
    }
}

// Get the head SHA of the repository (required for get_ref)
pub fn get_base(owner: &str, repo: &str, base_ref: &str) -> Result<String, ReqError> {
    let url = format!("https://api.github.com/repos/{}/{}/git/refs/heads/{}", owner, repo, base_ref);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Authorization", get_github_token())
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "acme-rs")
        .timeout(Duration::from_secs(5))
        .send()?;
    println!("get_base HTTP code: {:?}", response.status());
    let data: Content = response.json()?;
    Ok(data.object.sha)
}

// Check if named ref exists, if not (404 not found) create it
pub fn get_ref(owner: &str, repo: &str, gitref: String, head_sha: String) -> Result<String, ReqError> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/git/{}", owner, repo, gitref);
    let response = client
        .get(url)
        .header("Authorization", get_github_token())
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "SIGSTORE")
        .timeout(Duration::from_secs(120))
        .send()?;

    if response.status() == 404 {
        return create_ref(owner, repo, gitref, head_sha)
    } else {
        return Err(ReqError::BadRequest)
    }
}

// Create a ref
fn create_ref(owner: &str, repo: &str, gitref: String, head_sha: String) -> Result<String, ReqError> {
    println!("Creating ref: {}", gitref);
    let url: String = format!("https://api.github.com/repos/{}/{}/git/refs", owner, repo);
    let body = format!(
        r#"{{
            "ref": "{}",
            "sha": "{}"
        }}"#,
        gitref, head_sha
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header("Authorization", get_github_token())
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "SIGSTORE")
        .timeout(Duration::from_secs(120))
        .body(body)
        .send()?;
    println!("create_ref HTTP code {:?}", response.status());
    let data: Content = response.json()?;
    let ref_name = data.ref_;
    Ok(ref_name)
}

// Create a ref
fn update_ref(owner: &str, repo: &str, current_ref: String, new_sha: String) -> Result<String, ReqError> {
    let url: String = format!("https://api.github.com/repos/{}/{}/git/{}", owner, repo, current_ref);
    let body = format!(
        r#"{{
            "sha": "{}",
            "force": true          
        }}"#, new_sha
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .patch(url)
        .header("Authorization", get_github_token())
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "SIGSTORE")
        .timeout(Duration::from_secs(120))
        .body(body)
        .send()?;
    println!("update_ref HTTP code {:?}", response.status());
    let data: Content = response.json()?;
    let ref_name = data.ref_;
    Ok(ref_name)
}

// build the files into an array and create a git tree
pub fn create_tree(owner: &str, repo: &str, head_sha: String) -> Result<String, ReqError> {
    let url: String = format!("https://api.github.com/repos/{}/{}/git/trees", owner, repo);
    let body = format!(
        r#"{{
            "base_tree": "{}",
            "tree": [
                {{
                    "path": "README.md",
                    "mode": "100644",
                    "type": "blob",
                    "content": "Hello World!"
                }}
            ]
        }}"#,
        head_sha.to_string()
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header("Authorization", get_github_token())
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "SIGSTORE")
        .timeout(Duration::from_secs(120))
        .body(body)
        .send()?;

    println!("Create tree: {:?}", response.status());
    let data: TreeEntry = response.json()?;
    let tree_sha = data.sha;
    Ok(tree_sha)
}

pub fn get_parent(owner: &str, repo: &str, head_sha: String) -> Result<Vec<Parent>, ReqError> {
     let url: String = format!("https://api.github.com/repos/{}/{}/git/commits/{}", owner, repo, head_sha);
     let client = reqwest::blocking::Client::new();
     let response = client
        .get(url)
        .header("Authorization", get_github_token())
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "acme-rs")
        .timeout(Duration::from_secs(5))
        .send()?;
    println!("get_commit HTTP code: {:?}", response.status());
    let data: Commit = response.json()?;
    Ok(data.parents)
}

// create the commit
pub fn push_commit(current_ref: String, owner: &str, repo: &str,  tree_sha: String, parents: Vec<Parent>) -> Result<String, ReqError> {
    let _date_now = offset::Local::now();
    let url: String = format!("https://api.github.com/repos/{}/{}/git/commits", owner, repo);
    let mut parent_sha = Vec::new();
    for i in &parents {
        let i = i; // elements are immutable pointers
        parent_sha.push(&i.sha);
    }
    let body = format!(
        r#"{{
            "message": "Update script",
            "tree": "{}",
            "parents": {:?}
        }}"#,
        tree_sha, parent_sha
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header("Authorization", get_github_token())
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "SIGSTORE")
        .timeout(Duration::from_secs(120))
        .body(body)
        .send()?;
    println!("create_commit HTTP code: {:?}", response.status());
    let data: Commit = response.json()?;
    let new_sha = data.sha;

    // Attach the commit to the branch that was recently created
    let new_ref = update_ref(owner, repo, current_ref, new_sha)?;
    Ok(new_ref)

}

// push the pull request
pub fn create_pr(owner: &str, repo: &str,  title: &str, head: String, base: String, maintainer_can_modify: bool, draft: bool)  -> Result<String, ReqError> {
    let url: String = format!("https://api.github.com/repos/{}/{}/pulls", owner, repo);
    let body = format!(
        r#"{{
            "title": "{}",
            "head": "{}",
            "base": "{}",
            "maintainer_can_modify": {},
            "draft": {}
        }}"#,
        title, head, base, maintainer_can_modify, draft
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header("Authorization", get_github_token())
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "SIGSTORE")
        .timeout(Duration::from_secs(120))
        .body(body)
        .send()?;
    println!("create_pr HTTP code {:?}", response.status());
    let data: PullRequest = response.json()?;
    Ok(data.url)
}

// https://docs.rs/reqwest/latest/reqwest/struct.StatusCode.html

// match response.status() {
//         reqwest::StatusCode::OK => (),
//         reqwest::StatusCode::BAD_REQUEST => return Err(ReqError::BadRequest),
//         reqwest::StatusCode::UNAUTHORIZED => return Err(ReqError::AuthError),
//         reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE => {
//             return Err(ReqError::UnsupportedMediaType)
//         }
//         reqwest::StatusCode::TOO_MANY_REQUESTS => return Err(ReqError::TooManyRequest),
//         _ => return Err(ReqError::UnknownConnectionError),
// }
