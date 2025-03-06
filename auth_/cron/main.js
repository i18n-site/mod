import bantld from "@8v/bantld";
import heartbeat from "@8v/heartbeat/kind.js";

const KIND = "bantld";

Deno.cron(
  KIND, 
  "0 22 * * *", 
  heartbeat(KIND, bantld, 86400 * 5)
);
