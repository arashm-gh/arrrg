#!/usr/bin/env bash
set -euo pipefail

VERSION="v1.0.0"
URL="https://github.com/arashm-gh/arrrg/releases/download/$VERSION/arrrg-$VERSION-x86_64-unknown-linux-musl.tar.gz"

# Download and extract
curl -L "$URL" -o arrrg.tar.gz
tar -xzf arrrg.tar.gz

# Move to /usr/local/bin
sudo mv arrrg /usr/local/bin/
rm arrrg.tar.gz

echo "Installed arrrg! Try running: arrrg <query> <limit>"
