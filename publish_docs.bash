#!/usr/bin/env bash

# Generate documentation with Cargo and copy the generated HTML to the docs/
# directory. This will unstage any staged changes to keep the commit for
# updating the documentation modular.

git reset -- ./
mkdir -p docs/
cargo doc
cp -r target/doc/* docs/
echo "<head><meta http-equiv='refresh' content='0;URL=https://afnan.io/nib/nib/index.html'></head>" > docs/index.html
git add docs
git commit -m "[AUTOMATED] Update docs"
git push
