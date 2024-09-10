#!/bin/bash

# Run cargo format, watch for test changes
cargo fmt
cargo watch -x check -x test -x run