
#![allow(dead_code)]

mod api_client;
mod error;

use anyhow::Result;
use random_string::generate;

fn main() -> Result<()> {
    // get the SHA of the head
    let head_sha = api_client::get_base(
        "lukehinds",
        "acme",
        "main"
    )?;
    println!("head_sha: {:?}", head_sha);

    // Generate random branch name
    let charset = "1234567890";
    let generated_ref = format!("refs/heads/{}", generate(6, charset));

    let get_ref = api_client::get_ref("lukehinds",
        "acme",
        generated_ref,
        head_sha.clone()
    )?;
    println!("get_ref: {:?}", get_ref);

    let tree_sha = api_client::create_tree(
        "lukehinds",
        "acme",
        head_sha.clone()
    )?;
    println!("tree_sha: {:?}", tree_sha);

    // Returns a Vec<Content>
    let parents = api_client::get_parent(
        "lukehinds",
        "acme",
        head_sha.clone()
    )?;
    println!("parent: {:?}", parents[0].sha);
    println!("parent: {:?}", parents[1].sha);

    let commit_sha = api_client::push_commit(
        "lukehinds",
        "acme",
        tree_sha.clone(),
        parents
    )?;
    println!("commit_sha: {:?}", commit_sha);
    Ok(())
}
