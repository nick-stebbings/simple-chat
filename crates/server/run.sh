#!/bin/bash

# Set environment variables
export HOST="127.0.0.1"
export PORT=8080

# Run cargo format, watch for test changes
cargo watch -x run