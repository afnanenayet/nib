#!/usr/bin/env bash

# Generate documentation with Cargo and copy the generated HTML to the docs/
# directory

mkdir -p docs/
cargo doc
cp -r target/doc/* docs/
echo "<head><meta http-equiv='refresh' content='0;URL=https://afnan.io/nib/nib/index.html'></head>" > docs/index.html