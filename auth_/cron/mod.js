#!/usr/bin/env -S node --trace-uncaught --expose-gc --unhandled-rejections=strict
var domains, host, lastseen, o, ref, tld_set, update_ts, x;

import cJson from '@8v/curl/cJson.js';

import tld from '@8v/tld';

import punycode from 'punycode';

({
  t: update_ts,
  domains
} = (await cJson('https://raw.githubusercontent.com/7c/fakefilter/main/json/data.json')));

tld_set = new Set();

ref = Object.entries(domains);
for (x of ref) {
  [host, o] = x;
  ({lastseen} = o);
  if ((update_ts - lastseen) / 86400 < 365) {
    tld_set.add(punycode.toUnicode(tld(host)));
  }
}

console.log(tld_set, tld_set.size);
