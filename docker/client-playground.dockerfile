FROM petshop/node-tools:latest

# Copy source files
COPY ./docker/node-tools /home/node

# Build distribution files
RUN mkdir -p /home/node/dist \
    && chown node:node /home/node/dist \
    && npm run client-playground-build

EXPOSE 1234

CMD ["npm", "run", "client-playground-no-hmr"]
