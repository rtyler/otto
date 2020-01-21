# This Procfile is intended to be used by scripts/shoreman to run a development
# environment for Otto using locally built debug binaries

eventbus: RUST_LOG=info ./target/debug/otto-eventbus
auctioneer: RUST_LOG=debug ./target/debug/otto-auctioneer

# vim: ft=sh
