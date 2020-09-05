#!/usr/bin/env bash
set -ex

if [[ -z "$GITHUB_REF" ]]; then
  echo "GITHUB_REF must be set"
  exit 1
fi
TAG=${GITHUB_REF#*/tags/}

host=$(rustc -Vv | grep ^host: | sed -e "s/host: //g")
export CARGO_PROFILE_RELEASE_LTO=true
cargo build --bin hyeong --release
cd target/release
case $1 in
ubuntu* | macos*)
  asset="hyeong-$TAG-$host.tar.gz"
  tar czf ../../$asset hyeong
  ;;
windows*)
  asset="hyeong-$TAG-$host.zip"
  7z a ../../$asset hyeong.exe
  ;;
*)
  echo "OS should be first parameter, was: $1"
  ;;
esac
cd ../..

if [[ -z "$GITHUB_TOKEN" ]]; then
  echo "$GITHUB_TOKEN not set, skipping deploy."
else
  hub release edit -m "" --attach $asset $TAG
fi
