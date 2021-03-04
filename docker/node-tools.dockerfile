# DEPEND: docker pull node:15.11.0-buster
# <https://hub.docker.com/_/node/>
FROM node:15.11.0-buster

# Install package dependencies
COPY ./docker/node-tools/package.json /home/node/package.json
COPY ./docker/node-tools/package-lock.json /home/node/package-lock.json
WORKDIR /home/node
RUN npm i

# Copy source files after installation
COPY ./docker/node-tools /home/node

CMD ["/bin/bash"]
