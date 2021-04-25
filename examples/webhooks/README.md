# Receiving Webhooks

-   <https://docs.github.com/en/developers/webhooks-and-events/about-webhooks>
-   <https://ngrok.com/>

```shell
cargo make dist-build
cargo make compose build
cargo make compose up

# Use ngrok for external address
ngrok http 10000

# Copy HTTP forwarding addres and test using curl
curl -X POST -d "arg=foo" $NGROK_FORWARDING_ADDR/api.Example/Webhook

# In GitHub repository settings, add webhook with the URL above
# Once added the server should receive a ping event from GitHub
```
