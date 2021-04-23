# DEPEND: docker pull node:15.14.0-buster
# <https://hub.docker.com/_/node/>
FROM node:15.14.0-buster

# DEPEND: Install protocol buffers
# <https://github.com/protocolbuffers/protobuf>
RUN wget -O protoc.zip -q "https://github.com/protocolbuffers/protobuf/releases/download/v3.15.8/protoc-3.15.8-linux-x86_64.zip" \
    && unzip -o protoc.zip -d /usr/local bin/protoc \
    && unzip -o protoc.zip -d /usr/local 'include/*' \
    && chmod +x /usr/local/bin/protoc \
    && rm protoc.zip

# Install packages globally
# DEPEND: Update package versions
# <https://www.npmjs.com/package/ng-swagger-gen>
# <https://www.npmjs.com/package/@ngx-grpc/protoc-gen-ng>
RUN npm i -g ng-swagger-gen@2.3.1 @ngx-grpc/protoc-gen-ng@2.1.0 tslib

# Install package dependencies
COPY ./docker/node-tools/package.json /home/node/package.json
COPY ./docker/node-tools/package-lock.json /home/node/package-lock.json
WORKDIR /home/node
RUN npm i

# Copy source files
COPY ./docker/node-tools /home/node

# FIXME: Fixes root ownership issues when using protoc with local user
RUN chmod 777 -R /usr/local/include

CMD ["/bin/bash"]
