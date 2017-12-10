#!/bin/bash

set -e
bazel build \
    --crosstool_top=@mxebzl//tools/windows:toolchain --cpu=win64 \
    --workspace_status_command tools/buildstamp/get_workspace_status \
    -c opt :z2edit-windows

VERSION=$(grep BUILD_GIT_VERSION bazel-out/volatile-status.txt | cut -f2 -d' ')

cp bazel-bin/z2edit-windows.zip z2edit-windows-${VERSION}.zip
