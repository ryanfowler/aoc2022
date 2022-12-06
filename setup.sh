#!/bin/bash
set -e

day=$1

# Create the directory, if necessary.
if [[ ! -d ./src/day${day} ]]
then
    echo "Creating directory for day${day}"
    mkdir ./src/day"${day}"
fi

# Create the main.rs file, if necessary.
if [[ ! -f ./src/day${day}/main.rs ]]
then
    echo "Creating main.rs for day${day}"
    DAY_NUM=${day} envsubst < ./template.rs > ./src/day"${day}"/main.rs
fi

# Download and save the input, if necessary.
if [[ ! -f ./src/day${day}/input.txt ]]
then
    echo "Downloading input for day${day}"
    cookie=$(cat ./session.txt)
    curl -s -b "session=${cookie}" -o ./src/day"${day}"/input.txt https://adventofcode.com/2022/day/"${day/#0}"/input
fi

# Add bin to Cargo.toml, if necessary.
if ! grep -q "name = \"day${day}\"" ./Cargo.toml
then
    echo "Adding bin entry to Cargo.toml for day${day}"
    cat << EOF >> ./Cargo.toml

[[bin]]
name = "day${day}"
path = "src/day${day}/main.rs"
EOF
fi
