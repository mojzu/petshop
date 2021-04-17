# Cron Jobs

-   <https://devopsheaven.com/cron/docker/alpine/linux/2017/10/30/run-cron-docker-alpine.html>

```shell
cargo make dist-build

cd examples/cron
docker-compose build
docker-compose up
docker-compose down

# Wait up to a minute for example job to run and log output
```
