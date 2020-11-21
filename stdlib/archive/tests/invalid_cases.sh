#!/usr/bin/env bash

INVOCATION_FILE=tmp_archivetest_invocation_file.json

oneTimeTearDown() {
    rm -f $INVOCATION_FILE
}

test_fail_with_no_file() {
    assertFalse "step should fail when invoked without a param" archive-step
}

test_fail_with_malformed_file() {
    cat > $INVOCATION_FILE <<EOF
    {}
EOF
    assertFalse "step should fail when invoked with a bad invocation file" "archive-step $INVOCATION_FILE"
}

test_fail_with_absolute_path() {
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
        "artifacts" : "/etc/passwd"
    }
}
EOF

    assertFalse "step should fail when invoked with an absolute path" "archive-step $INVOCATION_FILE"
}

test_fail_with_path_traversal() {
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
        "artifacts" : "../../../"
    }
}
EOF

    assertFalse "step should fail when invoked with an absolute path" "archive-step $INVOCATION_FILE"
}

. $(dirname $0)/../../../contrib/shunit2/shunit2
