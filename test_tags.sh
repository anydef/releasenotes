#!/bin/bash
# Test the functionality with git tags
echo "Testing list-commits with git tags..."
cargo run -- list-commits -o anydef -r releasenotes -f v0.1.0 -t main

echo "Testing generate-release-notes with git tags..."
cargo run -- generate-release-notes -o anydef -r releasenotes -f v0.1.0 -t main