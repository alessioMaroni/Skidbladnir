#!/bin/bash

echo "Formatting workspace..."
cargo fmt --all

if git diff --quiet; then
    echo "No formatting changes to save."
else
    git add .
    git commit -m "style: format workspace with rustfmt"
    
    FMT_HASH=$(git rev-parse HEAD)
    
    echo "" >> .git-blame-ignore-revs
    echo "# rustfmt run" >> .git-blame-ignore-revs
    echo "$FMT_HASH" >> .git-blame-ignore-revs
    
    git add .git-blame-ignore-revs
    git commit --amend --no-edit
    
    echo "Formatting and ignore-revs completed in a single commit!"
fi