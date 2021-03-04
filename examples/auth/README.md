# auth

- <https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/ext_authz_filter>

```shell
cargo make dist-build

cd examples/auth
# Copy `oauth2-proxy.template.cfg` to `client-oauth2-proxy.cfg` and `server-oauth2-proxy.cfg` and edit as required
# Configuration fields to change: http_address, upstreams, provider
# <https://oauth2-proxy.github.io/oauth2-proxy/docs/configuration/oauth_provider>
# <https://oauth2-proxy.github.io/oauth2-proxy/docs/configuration/overview>

# Build and run example images
docker-compose build
docker-compose up
docker-compose down

# Open client playground at http://localhost:4180/

# Sign out/Sign in at http://localhost:4180/oauth2/sign_in
```
