# DEPEND: docker pull prom/prometheus:v2.25.2
FROM prom/prometheus:v2.25.2

COPY ./docker/prometheus/prometheus.yml /etc/prometheus/prometheus.yml
COPY ./docker/prometheus/alert.rules.yml /etc/prometheus/alert.rules.yml
