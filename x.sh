#!/usr/bin/env bash

echo "Checking if the cargo executable exists."
if ! command -v cargo &> /dev/null
then
  echo "Cargo executable not found."
  exit
fi

echo "Installing Diesel CLI"
cargo install diesel_cli --no-default-features --features postgres
