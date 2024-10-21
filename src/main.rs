use clap::{Parser, Subcommand};
use git2::{Config, Cred, PushOptions, RemoteCallbacks, Repository};
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
                let config = Config::open_default().unwrap();
                let user = config
                    .get_string("user.email")
                    .unwrap_or(String::from("tsc-cli"));

                if *create_repo {
                    let token = get_gh_auth_token(*reset).unwrap_or_default();
                    create_gh_repo(&token, &user, org, repo_name).unwrap_or_default();
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

                let mut remote = repo.find_remote("origin").unwrap();
                let mut callbacks = RemoteCallbacks::new();

                callbacks.credentials(|_url, _username_from_url, _allowed_types| {
                    Cred::ssh_key_from_agent("git").map_err(|e| e.into())
                });

                let mut push_options = PushOptions::new();
                push_options.remote_callbacks(callbacks);

                match remote.push(
                    &["refs/heads/main:refs/heads/main"],
                    Some(&mut push_options),
                ) {
                    Ok(_) => println!("Pushed successfully!"),
                    Err(e) => println!("Failed to push: {}", e),
                }
            }
            None => {}
        },
        None => {}
    }
}
