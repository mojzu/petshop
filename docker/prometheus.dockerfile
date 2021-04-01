# DEPEND: docker pull prom/prometheus:v2.26.0
FROM prom/prometheus:v2.26.0

COPY ./docker/prometheus/prometheus.yml /etc/prometheus/prometheus.yml
COPY ./docker/prometheus/alert.rules.yml /etc/prometheus/alert.rules.yml
