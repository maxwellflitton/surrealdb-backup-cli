#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..


if [ -d "./testing_data" ]; then
    echo "deleting testing data"
    rm -rf ./testing_data
fi

mkdir testing_data
cd testing_data
mkdir snapshots
git clone https://github.com/surrealdb/surrealdb.git

