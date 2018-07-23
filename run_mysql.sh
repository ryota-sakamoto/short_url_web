readonly MYSQL_ROOT_PASSWORD=root

docker run --rm -itd --name mariadb -e MYSQL_ROOT_PASSWORD=$MYSQL_ROOT_PASSWORD -p 3306:3360 mariadb
IP=$( docker exec -it mariadb ip a | grep global | sed -e "s/inet\s\+\(.*\)\/16.*/\1/")

while true
do
    mysql -h $IP -u root -p$MYSQL_ROOT_PASSWORD < table.sql
    if [ $? == 0 ]; then
        break
    fi
    sleep 10
done
