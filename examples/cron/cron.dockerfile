FROM petshop/server:latest

USER root

RUN echo "*       *       *       *       *       run-parts /etc/periodic/1min" >> /etc/crontabs/root

COPY ./example_job.sh /etc/periodic/1min/example_job
RUN chmod a+x /etc/periodic/1min/example_job

CMD ["crond", "-f", "-l", "8"]
