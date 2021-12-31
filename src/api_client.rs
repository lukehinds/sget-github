use crate::error::ReqError;
use std::env;
use std::time::Duration;
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
struct TreeEntry {
    sha: String,
    url: String,
    tree: Vec<TreeObject>,
    truncated: bool,
}

  #[derive(Serialize, Deserialize)]
struct TreeObject {
    path: String,
    mode: String,
    #[serde(rename = "type")]
    type_: String,
    size: u64,
    sha: String,
    url: String,
}

fn get_github_token() -> String {
    match env::var("GITHUB_AUTH_TOKEN") {
        Ok(val) => format!("Token {}", val),
        Err(_err) => ("").to_string(),
    }
}

// Get the head SHA of the repository (required for get_ref)
pub fn get_head(owner: &str, repo: &str) -> Result<String, ReqError> {
    let url = format!("https://api.github.com/repos/{}/{}/git/refs/heads", owner, repo);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Authorization", get_github_token())
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "acme-rs")
        .timeout(Duration::from_secs(5))
        .send()?;
    println!("{:?}", response.status());
    let data: Vec<Content> = response.json()?;
    let ref_sha = &data.last().unwrap().object.sha;
    Ok(ref_sha.to_string())
}

// Check if named ref exists, if not (404 not found) create it
pub fn get_ref(owner: &str, repo: &str, gitref: String, head_sha: String) -> Result<(), ReqError> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/git/{}", owner, repo, gitref);
    let response = client
        .get(url)
        .header("Authorization", get_github_token())
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "SIGSTORE")
        .timeout(Duration::from_secs(120))
        .send()?;
    println!("{:?}", response.status());

    // break this out into a function
    match response.status() {
        reqwest::StatusCode::OK => (),
        reqwest::StatusCode::BAD_REQUEST => return Err(ReqError::BadRequest),
        reqwest::StatusCode::UNAUTHORIZED => return Err(ReqError::AuthError),
        reqwest::StatusCode::NOT_FOUND => create_ref(owner, repo, gitref, head_sha)?,
        reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE => {
            return Err(ReqError::UnsupportedMediaType)
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => return Err(ReqError::TooManyRequest),
        _ => return Err(ReqError::UnknownConnectionError),
    }
    Ok(())
}

// Create a ref
fn create_ref(owner: &str, repo: &str, gitref: String, head_sha: String) -> Result<(), ReqError> {
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
    println!("Create ref{:?}", response.status());
    let data: Content = response.json()?;
    let ref_name = data.ref_;
    println!("{:?}", ref_name);
    Ok(())
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

// push the tree to the remote branch
fn prepare_commit() {

}

// push the commit
fn create_commit() {

}

// the ref is updated
fn update_ref() {

}

// create the pull request
fn create_pull_request() {

}

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


// https://docs.rs/reqwest/latest/reqwest/struct.StatusCode.html