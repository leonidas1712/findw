#!/bin/bash

if [ ! "$#" -eq 1 ]
then
  echo "Expecting 1 argument (<test_case_name or all>), got $#"
  exit 1
fi

arr=(input/*.in)
IFS=$'\n'

mkdir -p "./results/"

run_individual_test () {
  test_case="input/$1.in"
  result_file="results/$1.out"
  # run and output to res_file
  xargs ./target/release/findw < input/$1.in > $result_file

  correct_output_file="output/$1.out"

#   diff_file=results/$1$_diff.out
  diff -q <(sort $result_file) <(sort $correct_output_file)
  if [ $? -ne 0 ];
  then
    echo "FAILED: $1.in"
    diff $result_file $correct_output_file
  fi
}

# build for release
cargo b --release
printf "Built for release; running tests\n"

if [[ "$1" == "all" ]]; then
  test_case_name=$(basename $file_name .in)
  for file_name in ${arr[*]}; do
    test_case_name=$(basename $file_name .in)
    echo "Running test $test_case_name"
    run_individual_test $test_case_name

    printf ""
  done
else
  echo "Running test $1"
  run_individual_test $1
fi

unset IFS

