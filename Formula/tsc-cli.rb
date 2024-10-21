class TscCli < Formula
  desc "A CLI tool for 3 Sided Cube"
  homepage "https://github.com/3sidedcube/tsc-cli"
  url "https://github.com/3sidedcube/tsc-cli/releases/download/v0.1.0/tsc-cli-0.1.0-x86_64-apple-darwin.tar.gz"
  sha256 "27857d227207748b76ffece3508a9192558ff92f"
  version "0.1.0"

  def install
    bin.install "tsc-cli"
    bin.install_symlink "tsc-cli" => "tsc"
  end
end
