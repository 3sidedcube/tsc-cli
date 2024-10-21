class TscCli < Formula
  desc "A CLI tool for 3 Sided Cube"
  homepage "https://github.com/3sidedcube/tsc-cli"
  url "https://github.com/3sidedcube/tsc-cli/releases/download/v0.1.0/tsc-cli-0.1.0-aarch64-apple-darwin.tar.gz"
  sha256 "d01ca75f362f03356f8a2afd0baa269e1aa3e67f2b3d79a1e52621d86bf3f99f"
  version "0.1.0"

  def install
    bin.install "tsc-cli"
  end
end
