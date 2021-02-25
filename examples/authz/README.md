# authz

```shell
# Build base images
cargo make compose-build

cd examples/authz
# Copy `oauth2-proxy.template.cfg` to `client-oauth2-proxy.cfg` and `server-oauth2-proxy.cfg` and edit as required
# <https://oauth2-proxy.github.io/oauth2-proxy/docs/configuration/oauth_provider>
# <https://oauth2-proxy.github.io/oauth2-proxy/docs/configuration/overview>

# Build and run example images
docker-compose build
docker-compose up
docker-compose down
# ---

# Open client playground at http://localhost:4180/
```
