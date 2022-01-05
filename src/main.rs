
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

    // Based on PushCommit, this should update the object sha
    // currently it will be set the same as the head_sha, based on create tree
    // need to send reference and tree
    // This can be empty!
    // This needs to be returned as a Vector
    let parent = api_client::get_commit(
        "lukehinds",
        "acme",
        head_sha.clone()
    )?;
    // println!("parent: {:?}", parent);
    println!("parent sha: {:?}", parent);

    // let commit_sha = api_client::create_commit(
    //     "lukehinds",
    //     "acme",
    //     head_sha.clone(),
    //     tree_sha.clone(),
    // )?;
    // println!("commit_sha: {:?}", commit_sha);
    Ok(())
}
