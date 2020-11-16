#!/bin/sh

INVOCATION_FILE=tmp_gittest_invocation_file.json

oneTimeTearDown() {
    rm -f $INVOCATION_FILE
}

test_clone_simple() {
    cat > $INVOCATION_FILE<<EOF
    {
    "configuration" : {
        "pipeline" : "2265b5d0-1f70-46de-bf50-f1050e9fac9a",
        "uuid" : "5599cffb-f23a-4e0f-a0b9-f74654641b2b",
        "ipc" : "unix:///dev/null",
        "endpoints" : {
        }
    },
    "parameters" : {
        "url" : "https://git.brokenco.de/rtyler/otto-test-repository"
    }
}
EOF

    assertTrue "step should be able to clone the given url" "git-step $INVOCATION_FILE"
    assertTrue "step should have cloned the repo" "test -d otto-test-repository"
}

. $(dirname $0)/../../../contrib/shunit2/shunit2
