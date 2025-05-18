#!/bin/bash

# FIXME: this should be moved to an xtask but it works enough for now
cargo build --release --locked -Zbuild-std=core,alloc,panic_abort,std -Zbuild-std-features=panic_immediate_abort
