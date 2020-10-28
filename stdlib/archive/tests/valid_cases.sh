#!/bin/sh

INVOCATION_FILE=tmp_archivetest_invocation_file.yml
TAR_NAME=tmp_archivetest_tar

oneTimeTearDown() {
    rm -f $INVOCATION_FILE
    rm -f "${TAR_NAME}.tar.gz"

}

test_fail_with_file() {
    cat > $INVOCATION_FILE<<EOF
---
configuration:
  ipc: unix:///dev/null
parameters:
  artifacts: Cargo.toml
EOF

    assertTrue "step should do nothing with a single file" "archive-step $INVOCATION_FILE"
}

test_fail_with_dir() {
    cat > $INVOCATION_FILE<<EOF
---
configuration:
  ipc: unix:///dev/null
parameters:
  artifacts: $(dirname $0)
  name: ${TAR_NAME}
EOF

    assertTrue "step should create tarball with a directory" "archive-step $INVOCATION_FILE"
    assertTrue "file name ${TAR_NAME}.tar.gz not found" "test -f ${TAR_NAME}.tar.gz"
}

. $(dirname $0)/../../../contrib/shunit2/shunit2
