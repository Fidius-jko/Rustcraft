#!/usr/bin/env sh

rm -rf AppIcon.iconset/*
mkdir -p AppIcon.iconset
sips -z 16 16     ../icon_256x256.png --out AppIcon.iconset/icon_16x16.png
sips -z 32 32     ../icon_256x256.png --out AppIcon.iconset/icon_16x16@2x.png
sips -z 32 32     ../icon_256x256.png --out AppIcon.iconset/icon_32x32.png
sips -z 64 64     ../icon_256x256.png --out AppIcon.iconset/icon_32x32@2x.png
sips -z 128 128   ../icon_256x256.png --out AppIcon.iconset/icon_128x128.png
sips -z 256 256   ../icon_256x256.png --out AppIcon.iconset/icon_128x128@2x.png
cp ../icon_1024x1024.png AppIcon.iconset/icon_256x256.png
iconutil -c icns AppIcon.iconset
mkdir -p src/Game.app/Contents/Resources
mv AppIcon.icns src/Game.app/Contents/Resources/
