#!/bin/bash

version=$(cat version)
ARCH=$(uname -m)

echo $version
echo $ARCH

git pull
LDFLAGS="-static" cargo build --release
mkdir -p rustgrab.AppDir/usr/bin/
mkdir -p rustgrab.AppDir/usr/lib/x86_64-linux-gnu/
mkdir -p rustgrab.AppDir/lib/x86_64-linux-gnu/
mkdir -p releases/$version/
cp target/release/rustgrab rustgrab.AppDir/usr/bin/rustgrab
mkdir releases/$version
cp pkg/AppImage/AppRun rustgrab.AppDir/
cp pkg/AppImage/rustgrab.desktop rustgrab.AppDir/
cp pkg/AppImage/mail-send.svg rustgrab.AppDir/
if [ ! -f appimagetool-x86_64.AppImage ]; then
	curl https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage --output appimagetool-x86_64.AppImage
fi
chmod +x appimagetool-x86_64.AppImage
ldd target/release/rustgrab | grep "=> /" | awk '{print $3}' | xargs -I '{}' cp -v '{}' rustgrab.AppDir/usr/lib/x86_64-linux-gnu
./appimagetool-x86_64.AppImage -n rustgrab.AppDir releases/$version/rustgrab-$version-$ARCH.AppImage
