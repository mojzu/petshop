# DEPEND: docker pull prom/prometheus:v2.27.1
FROM prom/prometheus:v2.27.1

COPY ./docker/prometheus/prometheus.yml /etc/prometheus/prometheus.yml
COPY ./docker/prometheus/alert.rules.yml /etc/prometheus/alert.rules.yml
