#!/usr/bin/env bash

INVOCATION_FILE=tmp_archivetest_invocation_file.json

oneTimeTearDown() {
    rm -f $INVOCATION_FILE
}

test_pass_with_file() {
    cat > $INVOCATION_FILE<<EOF
    {
    "configuration" : {
        "pipeline" : "2265b5d0-1f70-46de-bf50-f1050e9fac9a",
        "uuid" : "5599cffb-f23a-4e0f-a0b9-f74654641b2b",
        "ipc" : "unix:///dev/null",
        "endpoints" : {
            "objects" : {
                "url" : "http://example.com"
            }
        }
    },
    "parameters" : {
        "artifacts" : "Cargo.toml"
    }
}
EOF

    echo "Hello world" > Cargo.toml

    assertTrue "step should do nothing with a single file" "archive-step $INVOCATION_FILE"
}

test_pass_with_dir() {
    TAR_NAME=tmp_archivetest_tar
    cat > $INVOCATION_FILE<<EOF
    {
    "configuration" : {
        "pipeline" : "2265b5d0-1f70-46de-bf50-f1050e9fac9a",
        "uuid" : "5599cffb-f23a-4e0f-a0b9-f74654641b2b",
        "ipc" : "unix:///dev/null",
        "endpoints" : {
            "objects" : {
                "url" : "http://example.com"
            }
        }
    },
    "parameters" : {
        "artifacts": "release",
        "name": "${TAR_NAME}"
    }
    }
EOF
    mkdir release
    touch release/one
    touch release/two

    assertTrue "step should create tarball with a directory" "archive-step $INVOCATION_FILE"
    assertTrue "file name ${TAR_NAME}.tar.gz not found" "test -f ${TAR_NAME}.tar.gz"
    rm -f "${TAR_NAME}.tar.gz"
}

. $(dirname $0)/../../../contrib/shunit2/shunit2
