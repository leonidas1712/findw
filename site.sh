PORT1=8000
PORT2=8001

cd ./test_site1
python3 -m http.server $PORT1 &
echo "Started test_site1 at $PORT1"

cd ../test_site2
python3 -m http.server $PORT2 &
echo "Started test_site2 at $PORT2"

echo "Started, waiting for SIGINT"

# TODO: do a more specific way to kill
catch_int() {
    echo "Caught int or term, stopping";
    kill $(pgrep python3) # kill procs started w python3
    exit
}

trap catch_int SIGINT

while true
do
:
done






