#!/bin/bash
## this file change the release version
VERSION=$1
if [ -z "$VERSION" ]; then
    echo "publish a new version"
    echo "./release.sh 0.1.7"
    exit 1
fi

sed -i "s/^version = .*/version = \"$VERSION\"/" Cargo.toml
sed -i "s/Current Version=.*/Current Version=$VERSION/" README.md

git add .
git commit -m"release version: $VERSION"
git tag $VERSION
git push
git push origin $VERSION
