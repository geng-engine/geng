#!/usr/bin/sh

set -e

cd /tmp
git clone https://github.com/tpoechtrager/osxcross
cd osxcross
wget -q https://github.com/phracker/MacOSX-SDKs/releases/download/10.15/MacOSX10.13.sdk.tar.xz --directory-prefix=tarballs
PORTABLE=yes UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh
