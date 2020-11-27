#!/usr/bin/env bash

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

test_clone_ref_branch() {
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
    rm -rf work-dir
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

    assertTrue "Reference repository should exist", "test -d ${cache_dir}/0884584c5aa4d28cbc4779fbc4cc9566625597528ee92e0092603e823057c1aa"
    rm -rf work-dir
}

test_repeat_clone_with_cache() {
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

    # Clone into one working directory with the "main" refspec

    mkdir work-dir
    pushd work-dir
        output=$(git-step $INVOCATION_FILE)
        assertTrue "step should be able to clone the given url: ${output}" $?
        assertTrue "step should have cloned the repo" "test -d otto-test-repository"
    popd
    rm -rf work-dir

    # Now that we're confident that the cache is primed, try to clone
    # a branch from that cached bare reference repo

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
        "url" : "https://git.brokenco.de/rtyler/otto-test-repository",
        "branch" : "test-branch"
    }
}
EOF

    mkdir work-dir
    pushd work-dir
        output=$(git-step $INVOCATION_FILE)
        assertTrue "step should be able to clone the given url: ${output}" $?
        assertTrue "step should have cloned the repo" "test -d otto-test-repository"
        assertTrue "step should have cloned the repo to the branch" "test -f otto-test-repository/this-is-a-branch"
    popd
    rm -rf work-dir
}

. $(dirname $0)/../../../contrib/shunit2/shunit2
