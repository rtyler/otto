#!/bin/sh

INVOCATION_FILE=tmp_archivetest_invocation_file.yml

oneTimeTearDown() {
    rm -f $INVOCATION_FILE
}

test_fail_with_no_file() {
    assertFalse "step should fail when invoked without a param" archive-step
}

test_fail_with_malformed_file() {
    cat > $INVOCATION_FILE <<EOF
---
# This is an invalid yaml file
EOF
    assertFalse "step should fail when invoked with a bad invocation file" "archive-step $INVOCATION_FILE"
}

test_fail_with_absolute_path() {
    cat > $INVOCATION_FILE<<EOF
---
configuration:
  ipc: unix:///dev/null
  endpoints: {}
parameters:
  artifacts: /etc/passwd
EOF

    assertFalse "step should fail when invoked with an absolute path" "archive-step $INVOCATION_FILE"
}

test_fail_with_path_traversal() {
    cat > $INVOCATION_FILE<<EOF
---
configuration:
  ipc: unix:///dev/null
  endpoints: {}
parameters:
  artifacts: ../../../
EOF

    assertFalse "step should fail when invoked with an absolute path" "archive-step $INVOCATION_FILE"
}

. $(dirname $0)/../../../contrib/shunit2/shunit2
