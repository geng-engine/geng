#!/usr/bin/sh

set -e

cd /tmp
git clone https://github.com/tpoechtrager/osxcross
cd osxcross

wget -q https://github.com/phracker/MacOSX-SDKs/releases/download/10.15/MacOSX10.13.sdk.tar.xz --directory-prefix=tarballs

# TODO: update clang instead? https://github.com/tpoechtrager/osxcross/issues/235
sed -i 's/apple-libtapi.git 1100.0.11/apple-libtapi.git 3cb307764cc5f1856c8a23bbdf3eb49dfc6bea48/g' build.sh;

PORTABLE=yes UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh
