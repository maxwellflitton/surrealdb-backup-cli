#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ../..

read -r -d '' SQL << EOM
USE ns test; 
USE db test;
SELECT * FROM user;
EOM

cd testing_data/surrealdb
echo "$SQL" | cargo run -- sql -u root -p root