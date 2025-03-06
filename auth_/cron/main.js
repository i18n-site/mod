import {env} from "node:process"

Deno.serve((_req) => new Response('env '+JSON.stringify(env.R_SENTINEL_NAME)));

// await import('@8v/bantld')
