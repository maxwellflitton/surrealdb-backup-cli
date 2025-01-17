#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ../..

read -r -d '' SQL << EOM
USE ns test; 
USE db test;
CREATE user:tobie SET name = 'Tobie';
CREATE user:jaime SET name = 'Jaime';
EOM

cd testing_data/surrealdb
echo "$SQL" | cargo run -- sql -u root -p root