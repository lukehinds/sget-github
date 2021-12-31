
#![allow(dead_code)]

mod api_client;
mod error;

use anyhow::Result;
use random_string::generate;

fn main() -> Result<()> {
    // get the SHA of the head
    let head_sha = api_client::get_head("lukehinds", "acme")?;
    println!("{:?}", head_sha);
    // Generate random branch name
    let charset = "1234567890";
    let gitref = format!("refs/heads/{}", generate(6, charset));
    let get_ref_response = api_client::get_ref("lukehinds", "acme", gitref, head_sha.clone())?;
    println!("{:?}", get_ref_response);
    let tree_sha = api_client::create_tree("lukehinds", "acme", head_sha.clone())?;
    println!("{:?}", tree_sha);

    Ok(())
}
