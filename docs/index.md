# Getting Started

## Installation

```bash
brew tap 3sidedcube/tap https://github.com/3sidedcube/tsc-cli.git

brew install 3sidedcube/tap/tsc-cli
```

## Usage

### Migrate repo to GitHub

If you intend to use repository auto-creation for a more seamless experience, you will need to create a fine-grained GitHub Personal Access Token (PAT).

1. From your GitHub, `Settings > Developer settings > Personal access tokens > Fine-grained tokens > Generate new token`.
2. Give it a name and set the owner to `3 Sided Cube`
3. Set an expiration of `30 days` (recommended, you can rotate this easily).
4. Optionally, give it a description.
5. Select `All repositories` for repository access.
6. Under repository permissions, set `Administration` to `read and write`.
7. Create the token and keep it somewhere safe for now.

```bash
# Run this inside the root of the project that you're migrating to GitHub

tsc-cli migrate github -c <repo_name>
```

_Note: if this is youre first time running this command, you will be prompted to enter your GitHub Personal Access Token (PAT)._
