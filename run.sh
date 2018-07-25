readonly MYSQL_ROOT_PASSWORD=root
readonly CONTAINER_NAME=short_url_db

function start() {
    docker stop $CONTAINER_NAME
    docker run --rm -itd --name $CONTAINER_NAME -e MYSQL_ROOT_PASSWORD=$MYSQL_ROOT_PASSWORD -p 3306:3360 mariadb
    IP=$( docker exec -it $CONTAINER_NAME ip a | grep global | sed -e "s/inet\s\+\(.*\)\/16.*/\1/" | sed -e "s/ //g")

    while true
    do
        mysql -h $IP -u root -p$MYSQL_ROOT_PASSWORD < table.sql
        if [ $? == 0 ]; then
            break
        fi
        echo "sleep"
        sleep 5
    done

    cargo build --release
    SHORT_URL_DB_IP=$IP ./target/release/short_url_web
}

function stop() {
    kill $(ps aux | grep "[s]hort_url_web" | awk '{print $2}')
    docker stop $CONTAINER_NAME
}

if [ $1 == "start" ]; then
    start
elif [ $1 == "stop" ]; then
    stop
fi