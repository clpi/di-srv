#!/bin/bash

if [ "$1" == "up" ]; then
    if [ "$2" == "rem" ]; then
        echo "Building Postgres remotely..."
        sudo podman build /usr/src/div/data/pg -t divdb
        echo "Running Postgres remotely..."
        sudo podman run -idt --rm 
            -p 5432:5432 
            -v /usr/src/div/data/pg/pgdata:/var/lib/postgresql 
            --env-file /usr/src/div/data/pg/pg.env 
            --name divdb 
            --hostname divdb 
            localhost/divdb
        echo "Running Postgres remotely."
    elif [ "$2" == "loc" ]; then
        echo "Building Postgres locally..."
        sudo podman build ../data/pg -t divdb
        echo "Running Postgres locally..."
        sudo podman run -idt --rm -p 5432:5432 -v ../data/pg/pgdata/:/var/lib/postgresql --env-file ../data/pg/pg.env --name divdb --hostname divdb localhost/divdb
        echo "Running Postgres locally."
    fi
elif [ "$1" == "log" ]; then
    if [ "$2" == "rem" ]; then
        psql -h io.div.is -p 5432 -U divadm divdb
    elif [ "$2" == "loc" ]; then
        psql -h localhost -p 5432 -U divadm divdb
    fi
elif [ "$1" == "init" ]; then
    if [ "$2" == "rem" ]; then
        psql -h io.div.is -p 5432 -U divadm -d divdb -a -q -f ../server/sql/schema.sql
    elif [ "$2" == "loc" ]; then
        psql -h localhost -p 5432 -U divadm -d divdb -a -q -f ../server/sql/schema.sql
    fi
fi
