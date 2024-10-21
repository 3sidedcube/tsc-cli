use std::io::{self, Write};

use keyring::{Entry, Result};

fn write_gh_auth_token(entry: Entry) -> Result<String> {
    print!("\nGitHub PAT Token: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    entry.set_password(&buffer)?;
    Ok(entry.get_password()?)
}

pub fn get_gh_auth_token(reset: bool) -> Result<String> {
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
