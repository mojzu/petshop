#!/usr/bin/env bash
cp /src/docker/sphinx/index.rst /src/index.rst \
    && cp /src/docker/sphinx/conf.py /src/conf.py \
    && make html \
    && cp -r /sphinx/build/html /src/docs/ \
    && rm -rf /src/index.rst /src/conf.py
