#!/bin/sh
set -e

BASE_URL="https://github.com/cognitive-engineering-lab/depot/releases/latest/download"
INSTALL_DIR=$HOME/.local/bin

download() {
  cd $(mktemp -d)
  
  echo 'Downloading prebuilt binary from Github...'
  wget "${BASE_URL}/$1.tar.gz"
  tar -xf $1.tar.gz

  mkdir -p $INSTALL_DIR
  mv depot $INSTALL_DIR/depot
}

ARCH=$(uname -m)
OS=$(uname)

pick_target() {
  echo "Selecting target for $ARCH / $OS..."

  if [ -n "$1" ]; then
    cargo install depot-js --locked --git https://github.com/cognitive-engineering-lab/depot/ --rev $1
    return
  elif [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
      download "x86_64-unknown-linux-gnu"
      return
    fi
  elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then
      download "aarch64-apple-darwin"
      return
    elif [ "$ARCH" = "x86_64" ]; then
      download "x86_64-apple-darwin"
      return
    fi
  fi

  echo 'Prebuilt binary not available, installing from source. This may take a few minutes.'
  cargo install depot-js --locked
}

pick_target $1

echo 'Depot installation is complete.'