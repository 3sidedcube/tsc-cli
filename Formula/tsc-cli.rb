class TscCli < Formula
  desc "A CLI tool for 3 Sided Cube"
  homepage "https://github.com/3sidedcube/tsc-cli"
  url "https://github.com/3sidedcube/tsc-cli/releases/download/v0.1.1/tsc-cli-0.1.1-aarch64-apple-darwin.tar.gz"
  sha256 "49dc3c9d58c0f396bb7288a41c9dfd0057ee64fecce9e2d103a38b446606f207"
  version "0.1.1"

  on_intel do
    url "https://github.com/3sidedcube/tsc-cli/releases/download/v0.1.1/tsc-cli-0.1.1-x86_64-apple-darwin.tar.gz"
    sha256 "8278c93dc0c5b99b3dd256c9c6bc327f2bba2e57d46f9ef48a74aeaa4f4c9262"
  end

  def install
    bin.install "tsc-cli"
  end
end
