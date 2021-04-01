# DEPEND: docker pull buildpack-deps:buster
# <https://hub.docker.com/_/buildpack-deps>
FROM buildpack-deps:buster as build

ENV OAUTH2_PROXY_VERSION="v7.1.1"

# DEPEND: Install Oauth2 Proxy
# <https://github.com/oauth2-proxy/oauth2-proxy>
RUN curl -fsSLO --compressed "https://github.com/oauth2-proxy/oauth2-proxy/releases/download/$OAUTH2_PROXY_VERSION/oauth2-proxy-$OAUTH2_PROXY_VERSION.linux-amd64.tar.gz" \
    && tar -xzf oauth2-proxy-$OAUTH2_PROXY_VERSION.linux-amd64.tar.gz --strip-components 1 -C /usr/local/bin/

FROM petshop/client-playground:latest as build2

# DEPEND: docker pull debian:10.9
# <https://hub.docker.com/_/debian>
FROM debian:10.9

# Install packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && apt-get autoremove -y && apt-get clean \
    && rm -rf /tmp/* /var/tmp/* \
    && rm -rf /var/lib/apt/lists/*

# Copy binaries
COPY --from=build /usr/local/bin/oauth2-proxy /usr/local/bin/oauth2-proxy
RUN chmod +x /usr/local/bin/oauth2-proxy

# Copy static files
COPY --from=build2 /home/node/dist /var/www/static

RUN mkdir -p /config
COPY ./docker/oauth2-proxy/oauth2-proxy.template.cfg /config/oauth2-proxy.cfg

# Create system user
RUN groupadd -r oa2proxy && useradd --no-log-init -r -g oa2proxy oa2proxy
USER oa2proxy:oa2proxy

CMD ["oauth2-proxy", "--config=/config/oauth2-proxy.cfg"]
