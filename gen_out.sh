#!/bin/bash

# generate output file given input file.in

if [ ! "$#" -eq 1 ]
then
  echo "Expecting 1 argument (<test_case_name | all>), got $#"
  exit 1
fi

cargo b --release

arr=(input/*.in)

if [[ "$1" == "all" ]]; then
  for file_name in ${arr[*]}; do
    # 0,1,jane...
    test_case_name=$(basename $file_name .in)
    echo "Generating test output/$test_case_name.out"
    xargs ./target/release/findw < $file_name > output/$test_case_name.out
    printf ""
  done
else
  echo "Generating output/$1.out..."
  xargs ./target/release/findw < input/$1.in > output/$1.out
fi