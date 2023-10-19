#!/bin/bash

if [ ! "$#" -gt 0 ]
then
  echo "Expecting list of test case names or 'all'"
  exit 1
fi

arr=(input/*.in)
IFS=$'\n'

mkdir -p "./results/"

run_individual_test () {
  test_case="input/$1.in"
  echo "Running test $test_case"
  result_file="results/$1.out"
  # run and output to res_file
  xargs ./target/release/findw < input/$1.in > $result_file

  correct_output_file="output/$1.out"

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
    run_individual_test $test_case_name

    printf ""
  done
else
  # echo "Running test $1"
  # run_individual_test $1
  for case in "$@"
  do
    run_individual_test "$case"
  done
fi

unset IFS

