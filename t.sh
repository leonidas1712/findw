echo "Started, waiting for SIGINT"

catch_int() {
    echo "Caught int or term, stopping";
    exit
}

trap catch_int SIGINT

while true
do
:
done


