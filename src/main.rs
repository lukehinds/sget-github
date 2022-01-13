
#![allow(dead_code)]

mod api_client;
mod error;

use anyhow::Result;
use random_string::generate;

fn main() -> Result<()> {
    let owner = "jyotsna-penumaka";
    let repo = "acme";
    let base = "main";
    let maintainer_can_modify = true;
    let draft = false;
    
    // get the SHA of the head
    let head_sha = api_client::get_base(
        owner,
        repo,
        base
    )?;
    println!("head_sha: {:?}", head_sha);

    // Generate random branch name
    let charset = "1234567890";
    let branch_name = generate(6, charset);
    let generated_ref = format!("refs/heads/{}", branch_name);

    let get_ref = api_client::get_ref(owner,
        repo,
        generated_ref,
        head_sha.clone()
    )?;
    println!("get_ref: {:?}", get_ref);

    let tree_sha = api_client::create_tree(
        owner,
        repo,
        head_sha.clone()
    )?;
    println!("tree_sha: {:?}", tree_sha);

    let parents = api_client::get_parent(
        owner,
        repo,
        head_sha.clone()
    )?;
    // println!("parent: {:?}", parents[1].sha);

    // Make a commit
    let commit_sha = api_client::push_commit(
        get_ref,
        owner,
        repo,
        tree_sha.clone(),
        parents
    )?;
    println!("commit_sha: {:?}", commit_sha);

    // Create a pull request
    let pr_title = format!("This pull request created by {} merges {} into {}", owner, branch_name, base);

    let pr_url = api_client::create_pr(
        owner, 
        repo,
        &pr_title, 
        branch_name.to_string(), 
        base.to_string(), 
        maintainer_can_modify,
        draft
    )?;  
    println!("pr_url: {:?}", pr_url);
    
    Ok(())
}
