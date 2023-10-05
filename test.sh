#!/bin/bash

if [ ! "$#" -eq 1 ]
then
  echo "Expecting 1 argument (<test_case_name or all>), got $#"
  exit 1
fi

arr=(input/**/*.in)
IFS=$'\n'

mkdir -p "./results/"

echo "ok"

# Do the same for clang too
run_individual_test () {
  test_case="input/$1.in"
  result_file="results/$1_${NUM_THREADS}.out"
  ./build/iom_gnu $test_case $result_file $NUM_THREADS > /dev/null
  correct_output_file="output/$1.out"
  if [ ! -e "$correct_output_file" ]; then
    echo "Generating golden output for $1"
    ./iom_sequential $test_case $correct_output_file 1
  fi
  diff -q $result_file $correct_output_file > /dev/null
  if [ $? -ne 0 ];
  then
    echo "FAILED: $1"
  fi
}

# if [[ "$1" == "all" ]]; then
#   for file_name in ${arr[*]}; do
#     test_case_name=$(basename $file_name .in)
#     run_individual_test $test_case_name
#   done
# else
#   run_individual_test $1
# fi

unset IFS

