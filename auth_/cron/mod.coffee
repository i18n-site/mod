#!/usr/bin/env coffee

> @8v/curl/cJson.js
  @8v/tld
  punycode

{t:update_ts, domains} = await cJson(
  'https://raw.githubusercontent.com/7c/fakefilter/main/json/data.json'
)

tld_set = new Set

for [host,o] from Object.entries domains
  {lastseen} = o
  if (update_ts - lastseen)/86400 < 365
    tld_set.add punycode.toUnicode(tld(host))

console.log tld_set, tld_set.size
