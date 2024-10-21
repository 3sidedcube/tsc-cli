class TscCli < Formula
  desc "A CLI tool for 3 Sided Cube"
  homepage "https://github.com/3sidedcube/tsc-cli"
  url "https://github.com/3sidedcube/tsc-cli/releases/download/v0.1.0/tsc-cli-0.1.0-aarch64-apple-darwin.tar.gz"
  sha256 "d01ca75f362f03356f8a2afd0baa269e1aa3e67f2b3d79a1e52621d86bf3f99f"
  version "0.1.0"

  on_intel do
    url "https://github.com/3sidedcube/tsc-cli/releases/download/v0.1.0/tsc-cli-0.1.0-x86_64-apple-darwin.tar.gz"
    sha256 "86054de1c7c1570cbc40604d321a774ebad35d1741198876718cda9da3ca6700"
  end

  def install
    bin.install "tsc-cli"
  end
end
