FROM petshop/oauth2-proxy:latest

COPY ./server-oauth2-proxy.cfg /config/oauth2-proxy.cfg
