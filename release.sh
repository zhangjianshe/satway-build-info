#!/bin/bash
## this file change the release version
VERSION=$1
if [ -z "$VERSION" ]; then
    echo "THIS TOOL IS USED TO PUBLISH A NEW VERSION TO GITHUB TAG."
    echo "  "
    echo "$ ./release.sh version_number"
    ## compute next release version
    NEXT_VERSION=$(cargo pkgid | sed -E 's/.*@([0-9]+\.[0-9]+)\.([0-9]+)/\1.\2/' | awk -F'.' '{print $1"."$2"."($3+1)}')
    echo "next public version would be : $NEXT_VERSION"
    echo "please try                     ./release.sh $NEXT_VERSION"
    exit 1
fi

sed -i "s/^version = .*/version = \"$VERSION\"/" Cargo.toml
sed -i "s/Current Version=.*/Current Version=$VERSION/" README.md
sed -i "s/cargo add satway_build@.*/cargo add satway_build@$VERSION/" README.md
cargo build
git add .
git commit -m"release version: $VERSION"
git tag $VERSION
git push
git push origin $VERSION
