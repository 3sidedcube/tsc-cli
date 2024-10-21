use clap::{Parser, Subcommand};
use git2::Repository;
use modules::remote::get_gh_auth_token;

mod modules;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Platform {
    #[command(name = "github")]
    GitHub {
        #[arg(short, long)]
        create_repo: bool,

        #[arg(short, long, default_value = "3sidedcube")]
        org: String,

        #[arg(help = "Name of the repository")]
        repo_name: String,
    },
}

#[derive(Subcommand)]
enum Commands {
    Migrate {
        #[command(subcommand)]
        platform: Option<Platform>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Migrate { platform }) => match platform {
            Some(Platform::GitHub {
                create_repo,
                org,
                repo_name,
            }) => {
                let url = format!("git@github.com:{}/{}.git", org, repo_name);

                if *create_repo {
                    let token = get_gh_auth_token(false).unwrap_or_default();
                    println!("Token: {}", token)
                }

                let repo = Repository::open(".").unwrap();
                match repo.remote_set_url("origin", &url) {
                    Ok(_) => {
                        println!("New remote URL set.")
                    }
                    Err(_) => {
                        println!("Failed to set new remote URL.")
                    }
                }

                println!("Reconfigure current repository to new repo")
            }
            None => {}
        },
        None => {}
    }
}
