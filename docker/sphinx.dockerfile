# DEPEND: docker pull python:3.9.4-buster
# <https://hub.docker.com/_/python>
FROM python:3.9.4-buster

RUN pip install sphinx myst-parser sphinx-rtd-theme

RUN mkdir -p /sphinx/build
COPY ./docker/sphinx/Makefile /sphinx/Makefile
COPY ./docker/sphinx/make-html.sh /usr/local/bin/sphinx-make-html
RUN chmod +x /usr/local/bin/sphinx-make-html

# FIXME: Fixes build issues when running as user
RUN chmod 777 -R /sphinx

WORKDIR /sphinx

EXPOSE 8079
ENTRYPOINT ["bash"]
