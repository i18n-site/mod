#!/usr/bin/env -S node --trace-uncaught --expose-gc --unhandled-rejections=strict
var domains, host, lastseen, o, ref, t, x;

import cJson from '@8v/curl/cJson.js';

import {
  parse
} from 'tldts';

({t, domains} = (await cJson('https://raw.githubusercontent.com/7c/fakefilter/main/json/data.json')));

ref = Object.entries(domains);
for (x of ref) {
  [host, o] = x;
  ({lastseen} = o);
  if ((t - lastseen) / 86400 < 365) {
    console.log(host, parse(host).publicSuffix);
  }
}
