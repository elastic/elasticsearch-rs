# Cloudflare workers

Example using [Cloudflare workers](https://workers.cloudflare.com/). Workers are Javascript function-as-a-service that can run WebAssembly. The [`wrangler` CLI](https://developers.cloudflare.com/workers/) provides everything to compile workers, test them locally and deploy them to Cloudflare.

After setting up the `wrangler` CLI, run locally with:

```
wrangler secret put ES_URL
<enter elasticsearch url>

wrangler secret put ES_LOGIN
<enter elasticsearch login>

wrangler secret put ES_PASSWORD
<enter elasticsearch password>

wrangler dev
```

