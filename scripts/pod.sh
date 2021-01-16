run_redis() {
    docker run -p 6379:6379 -d                  \
        -v $PWD/redis-data:/bitnami/redis/data  \
        --name redis_cont                       \
        bitnami/redis:latest # <-- Redis image
}

run_api() {
    podman build . -t divapi && podman run -t divapi
}
