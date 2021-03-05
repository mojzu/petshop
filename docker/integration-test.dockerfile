FROM petshop/node-tools:latest

# Copy source files
COPY ./docker/node-tools /home/node

# Copy wait-for-it script
COPY ./docker/server/wait-for-it.sh /usr/local/bin/wait-for-it
RUN chmod +x /usr/local/bin/wait-for-it

CMD ["npm", "run", "integration-test"]
