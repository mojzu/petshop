#!/usr/bin/env bash
cp /src/docker/sphinx/index.rst /src/index.rst \
    && cp /src/docker/sphinx/conf.py /src/conf.py \
    && make html \
    && mkdir -p /src/dist/manual \
    && rm -rf /src/dist/manual/html \
    && cp -r /sphinx/build/html /src/dist/manual/html \
    && rm -rf /src/index.rst /src/conf.py
