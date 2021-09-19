#!/bin/bash

mkdir -p dist
rm -rf ./dist/*

# Build `next`
cd next
mdbook build -d ../dist/
cd ../

# Loop through all directories that aren't `common` or `next` (or `dist` of course)
for d in */ ; do
    if [[ $d == "common/" || $d == "next/" || $d == "dist/" ]]; then
        continue
    else
        cd $d
        mdbook build -d ../dist/$d
        cd ../
    fi
done

# Copy in the redirection for the latest stable version
cp stable.html dist/stable.html
