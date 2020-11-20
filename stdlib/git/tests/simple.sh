#!/bin/sh

INVOCATION_FILE=$PWD/tmp_gittest_invocation_file.json

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

    output=$(git-step $INVOCATION_FILE)
    assertTrue "step should be able to clone the given url: ${output}" $?
    assertTrue "step should have cloned the repo" "test -d otto-test-repository"
    rm -rf otto-test-repository
}

test_clone_ref_tag() {
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
        "url" : "https://git.brokenco.de/rtyler/otto-test-repository",
        "branch" : "test-branch"
    }
}
EOF

    output=$(git-step $INVOCATION_FILE)
    echo $output
    assertTrue "step should be able to clone the given url: ${output}" $?
    assertTrue "step should have cloned the repo" "test -d otto-test-repository"
    assertTrue "step should have cloned the repo to the branch" "test -f otto-test-repository/this-is-a-branch"
    rm -rf otto-test-repository
}

test_clone_into() {
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
        "url" : "https://git.brokenco.de/rtyler/otto-test-repository",
        "into" : "."
    }
}
EOF

    mkdir work-dir
    pushd work-dir
        output=$(git-step $INVOCATION_FILE)
        assertTrue "step should be able to clone the given url: ${output}" $?
        assertTrue "step should have cloned the repo into $PWD" "test -f README.adoc"
    popd
}
test_clone_with_cache() {
    cache_dir="$PWD/caches"

    cat > $INVOCATION_FILE<<EOF
    {
    "configuration" : {
        "pipeline" : "2265b5d0-1f70-46de-bf50-f1050e9fac9a",
        "uuid" : "5599cffb-f23a-4e0f-a0b9-f74654641b2b",
        "cache" : "${cache_dir}",
        "ipc" : "unix:///dev/null",
        "endpoints" : {
        }
    },
    "parameters" : {
        "url" : "https://git.brokenco.de/rtyler/otto-test-repository"
    }
}
EOF

    mkdir work-dir
    pushd work-dir
        output=$(git-step $INVOCATION_FILE)
        assertTrue "step should be able to clone the given url: ${output}" $?
    popd

}

. $(dirname $0)/../../../contrib/shunit2/shunit2
