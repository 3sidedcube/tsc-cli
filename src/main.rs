use clap::{Parser, Subcommand};
use git2::Repository;
use modules::remote::{create_gh_repo, get_gh_auth_token};

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

        #[arg(short, long)]
        reset: bool,
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
                reset,
            }) => {
                let url = format!("git@github.com:{}/{}.git", org, repo_name);

                if *create_repo {
                    let token = get_gh_auth_token(*reset).unwrap_or_default();
                    create_gh_repo(&token, org, repo_name).unwrap_or_default();

                    return;
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
