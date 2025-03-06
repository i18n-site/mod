#!/usr/bin/env coffee

> @8v/curl/cJson.js
  tldts > parse

{t, domains} = await cJson(
  'https://raw.githubusercontent.com/7c/fakefilter/main/json/data.json'
)

for [host,o] from Object.entries domains
  {lastseen} = o
  if (t - lastseen)/86400 < 365
    console.log host, parse(host).publicSuffix
