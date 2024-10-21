use std::io::{self, Write};

use keyring::Entry;
use serde_json::json;

fn write_gh_auth_token(entry: Entry) -> keyring::Result<String> {
    print!("\nGitHub PAT Token: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    entry.set_password(buffer.trim())?;
    Ok(entry.get_password()?)
}

pub fn get_gh_auth_token(reset: bool) -> keyring::Result<String> {
    let entry = Entry::new("tsc-cli", "gh-auth-token")?;

    match entry.get_password() {
        Ok(p) => {
            if reset {
                return Ok(write_gh_auth_token(entry)?);
            }

            Ok(p)
        }
        Err(_) => Ok(write_gh_auth_token(entry)?),
    }
}

pub fn create_gh_repo(token: &str, user: &str, org: &str, repo_name: &str) -> reqwest::Result<()> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.github.com/orgs/{}/repos", org);

    let data = json!({
        "name": &repo_name,
        "private": true,
        "visibility": "private",
        "has_issues": false,
        "has_projects": false,
        "has_wiki": false,
        "has_downloads": false,
        "allow_merge_commit": false,
        "allow_rebase_merge": false,
        "delete_branch_on_merge": true,
    });

    client
        .post(&url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", user)
        .header("X-GitHub-Api-Version", "2022-11-28")
        .bearer_auth(token)
        .json(&data)
        .send()?;

    Ok(())
}
