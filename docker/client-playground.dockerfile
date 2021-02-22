FROM petshop/node-tools:latest

EXPOSE 1234

CMD ["npm", "run", "client-playground-no-hmr"]
