#!/bin/bash

# generate perf for the .in
if [ ! "$#" -eq 1 ]
then
  echo "Expecting 1 argument (<test_case_name>), got $#"
  exit 1
fi

cargo b --release
# hyperfine "xargs ./target/release/findw < input/$1.in" -r 3


# TODO: make this work for all
arr=(input/*.in)

RUNS=3

if [[ "$1" == "all" ]]; then
  for file_name in ${arr[*]}; do
    # 0,1,jane...
    test_case_name=$(basename $file_name .in)
    perf_file="perf/$test_case_name.perf"
    echo "Generating perf: $perf_file"
   (hyperfine "xargs ./target/release/findw < input/$test_case_name.in" -r 3) > $perf_file
    printf ""
  done
else
  echo "Generating perf/$1.perf..."
#   xargs ./target/release/findw < input/$1.in > output/$1.out
fi

