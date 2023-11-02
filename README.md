#  <div  align="center"> findw </div>

<p  align="center"> Investigating a recursive, parallel, space-optimal adaptation of Unix’s find command for webpages. </p>

## Running tests
```
./test.sh 0 1 # run specific tests
./test.sh all # run all tests
```

## Getting performance metrics
```
./perf.sh 0 3 # get metrics for case 0 with 3 runs - outputs to perf/0.perf
./perf.sh all # metrics for all cases
```

## Generate output to test against
```
./gen_out.sh 0
./gen_out.sh all
```