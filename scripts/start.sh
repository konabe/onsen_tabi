#!/bin/sh
cargo install diesel_cli --no-default-features --features "mysql"
diesel migration run