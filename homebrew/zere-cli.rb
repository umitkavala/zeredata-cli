class ZereCli < Formula
  desc "CLI tool for ZereData - synthetic data generation for robotics"
  homepage "https://github.com/umitkavala/zeredata-cli"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/umitkavala/zeredata-cli/releases/download/cli-v#{version}/zere-darwin-arm64"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/umitkavala/zeredata-cli/releases/download/cli-v#{version}/zere-darwin-amd64"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/umitkavala/zeredata-cli/releases/download/cli-v#{version}/zere-linux-arm64"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/umitkavala/zeredata-cli/releases/download/cli-v#{version}/zere-linux-amd64"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  def install
    bin.install "zere-#{OS.kernel_name.downcase}-#{Hardware::CPU.arch}" => "zere"
  end

  test do
    assert_match "zere", shell_output("#{bin}/zere --version")
  end
end
