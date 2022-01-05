use crate::error::ReqError;
use chrono::Utc;
use std::env;
use std::time::Duration;
use serde::{Serialize, Deserialize};

// import ureq Response struct

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

#[derive(Serialize, Deserialize,)]
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
    signature: String,
    payload: String,
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
    // let data: Vec<Content> = response.json()?;
    let data: Content = response.json()?;
    println!("get_base data: {:?}", data.object.sha);
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
                    "content": "Hello World"
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

// create the pull request
pub fn push_commit(owner: &str, repo: &str,  tree_sha: String, parents: Vec<Parent>) -> Result<(), ReqError> {
    let _date_now = chrono::offset::Local::now();
    let url: String = format!("https://api.github.com/repos/{}/{}/git/commits", owner, repo);
    let body = format!(
        r#"{{
            "message": "Update script",
            "tree": "{}",
            "parents": [
                "{:?}"
            ]
        }}"#,
        tree_sha, parents
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
    println!("create_commit data: {:?}", data);
    Ok(())

}

// push the pull request
pub fn create_pr() {

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
