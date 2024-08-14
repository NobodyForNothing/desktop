#!/bin/bash

LIBRARY_NAME="fast_rss_data"
CURR_VERSION=$LIBRARY_NAME-v`awk '/^version: /{print $2}' packages/$LIBRARY_NAME/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_$LIBRARY_NAME/ios/flutter_$LIBRARY_NAME.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_$LIBRARY_NAME/macos/flutter_$LIBRARY_NAME.podspec
rm packages/flutter_$LIBRARY_NAME/macos/*.bak packages/flutter_$LIBRARY_NAME/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(LibraryVersion \"$CURR_VERSION\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/flutter_$LIBRARY_NAME/$CMAKE_PLATFORM/CMakeLists.txt
    rm packages/flutter_$LIBRARY_NAME/$CMAKE_PLATFORM/*.bak
done

git add packages/flutter_$LIBRARY_NAME/