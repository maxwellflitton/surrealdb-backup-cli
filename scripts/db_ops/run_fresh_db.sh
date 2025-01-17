#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ../..

cd testing_data/surrealdb
cargo run --features storage-rocksdb -- start --allow-all -u root -p root rocksdb:..//snapshots/fresh_db
