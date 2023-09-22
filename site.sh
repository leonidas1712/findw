PORT1=8000
PORT2=8001
cd ./test_site1
python3 -m http.server $PORT1 &
echo "Started test_site1 at $PORT1"

cd ../test_site2
python3 -m http.server $PORT2 &
echo "Started test_site2 at $PORT2"







