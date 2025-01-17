#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ../..

# Copy over new data
cp -r testing_data/snapshots/fresh_db testing_data/snapshots/fresh_db_copy/
rm testing_data/snapshots/fresh_db_copy/LOCK

# pack the data
cargo run -- pack -d testing_data/snapshots/fresh_db_copy/ -t testing_data/snapshots/fresh_package.sst
rm -rf testing_data/snapshots/fresh_db_copy/

# unpack the data
cargo run -- unpack -d testing_data/snapshots/fresh_db_unpack/ -t ./testing_data/snapshots/fresh_package.sst
rm testing_data/snapshots/fresh_db_unpack/LOCK

# rm -rf testing_data/snapshots/fresh_db_unpack/
rm ./testing_data/snapshots/fresh_package.sst
