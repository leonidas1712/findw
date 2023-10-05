#!/bin/bash

# generate output file given input file.in

if [ ! "$#" -eq 1 ]
then
  echo "Expecting 1 argument (<test_case_name>), got $#"
  exit 1
fi

cargo b --release
xargs ./target/release/findw < input/$1.in > output/$1.out