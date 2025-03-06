import bantld from "@8v/bantld";
import heartbeat from "@8v/heartbeat/kind.js";
import nextMinute from "@8v/cron/nextMinute.js";

const KIND = "bantld";

Deno.cron(
  KIND, 
  nextMinute(), 
  heartbeat(KIND, bantld, 86400 * 5)
);
