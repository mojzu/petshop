# DEPEND: docker pull golang:1.16.0-buster
# <https://hub.docker.com/_/golang>
FROM golang:1.16.0-buster

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# Install packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends unzip \
	&& rm -rf /var/lib/apt/lists/*

# DEPEND: Install protocol buffers
# <https://github.com/protocolbuffers/protobuf>
RUN wget -O protoc.zip -q "https://github.com/protocolbuffers/protobuf/releases/download/v3.15.4/protoc-3.15.4-linux-x86_64.zip" \
    && unzip -o protoc.zip -d /usr/local bin/protoc \
    && unzip -o protoc.zip -d /usr/local 'include/*' \
    && chmod +x /usr/local/bin/protoc \
    && rm protoc.zip

# DEPEND: Install grpc-gateway
# <https://github.com/grpc-ecosystem/grpc-gateway>
#
# FIXME: Update go.sum by deleting file commenting out COPY below, then after build remember
# to copy the file back by running `cargo make go-tools -- cp /go/src/go.sum /src/docker/go-tools/go.sum`
COPY ./docker/go-tools/go.mod /go/src/go.mod
COPY ./docker/go-tools/tools.go /go/src/tools.go
COPY ./docker/go-tools/go.sum /go/src/go.sum
RUN (cd /go/src && go mod tidy) \
    && (cd /go/src && go install \
        github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway \
        github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2 \
        google.golang.org/protobuf/cmd/protoc-gen-go \
        google.golang.org/grpc/cmd/protoc-gen-go-grpc)

# DEPEND: Install gRPC web generator
# <https://github.com/grpc/grpc-web>
RUN wget -O /usr/local/bin/protoc-gen-grpc-web -q "https://github.com/grpc/grpc-web/releases/download/1.2.1/protoc-gen-grpc-web-1.2.1-linux-x86_64" \
    && chmod +x /usr/local/bin/protoc-gen-grpc-web

# FIXME: Fixes root ownership issues when using protoc with local user
RUN chmod 777 -R /usr/local/include /go/src

RUN mkdir -p /src
WORKDIR /src
CMD ["/bin/bash"]
