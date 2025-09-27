#!/usr/bin/env bash
set -e

VERSION="v1.0.0"
URL="https://github.com/arashm-gh/arrrg/releases/download/$VERSION$/arrrg-$VERSION$-x86_64.tar.gz"
curl -L "$URL$" | tar -xz
sudo mv arrrg /usr/local/bin/
ehco "Installed arrrg! Try running arrrg <query> <limit>"
