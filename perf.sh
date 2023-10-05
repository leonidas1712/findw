#!/bin/bash
# generate perf for the .in
if [ ! "$#" -eq 2 ]
then
  echo "Expecting 2 arguments (<test_case_name> <runs>), got $#"
  exit 1
fi

cargo b --release

arr=(input/*.in)

RUNS=$2

if [[ "$1" == "all" ]]; then
  for file_name in ${arr[*]}; do
    # 0,1,jane...
    test_case_name=$(basename $file_name .in)
    perf_file="./perf/$test_case_name.perf"
    echo "Generating perf: $perf_file"
   (hyperfine "xargs ./target/release/findw < input/$test_case_name.in" -r $RUNS) > $perf_file
    printf ""
  done
else
  perf_file="./perf/$1.perf"
  echo "Generating $perf_file.."
  (hyperfine "xargs ./target/release/findw < input/$1.in" -r $RUNS) > $perf_file
fi

