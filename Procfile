# This Procfile is intended to be used by scripts/shoreman to run a development
# environment for Otto using locally built debug binaries
#

object-store: OTTO_OBJECT_DIR=tmp/objects RUST_LOG=debug ./target/debug/otto-object-store
orchestrator: RUST_LOG=debug STEPS_DIR=$PWD/tmp PATH=$PWD/target/debug:$PATH otto-local-orchestrator
parser: RUST_LOG=debug ./target/debug/otto-parser
reldata: RUST_LOG=debug ./target/debug/otto-reldata

# vim: ft=sh
