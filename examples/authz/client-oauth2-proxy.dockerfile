FROM petshop/oauth2-proxy:latest

COPY ./client-oauth2-proxy.cfg /config/oauth2-proxy.cfg
