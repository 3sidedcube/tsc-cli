class TscCli < Formula
  desc "A CLI tool for 3 Sided Cube"
  homepage "https://github.com/3sidedcube/tsc-cli"
  url "https://github.com/3sidedcube/tsc-cli/releases/download/v0.1.0/tsc-cli-0.1.0-x86_64-apple-darwin.tar.gz", :using => GitHubPrivateRepositoryReleaseDownloadStrategy
  sha256 "86054de1c7c1570cbc40604d321a774ebad35d1741198876718cda9da3ca6700"
  version "0.1.0"

  def install
    bin.install "tsc-cli"
  end
end
