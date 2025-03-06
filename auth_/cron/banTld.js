#!/usr/bin/env -S node --trace-uncaught --expose-gc --unhandled-rejections=strict
var DAY, FAKEFILTER, KEY, PROXY, UPDATE, WEEK;

import reqJson from '@3-/req/reqJson.js';

import reverse from '@3-/reverse';

import pack from '@3-/msgpack/pack.js';

import unpack from '@3-/msgpack/unpack.js';

import R from '@3-/redis/R.js';

import {
  PWD
} from './conf';

import {
  join
} from 'path';

import tld from 'tld-extract';

import punycode from 'punycode';

PROXY = 'https://mirror.ghproxy.com/';

DAY = 86400;

WEEK = DAY * 7;

KEY = 'banTld';

UPDATE = KEY + ':update';

FAKEFILTER = KEY + ':fakefilter';

export const main = async() => {
  var diff, domains, fakefilter, host, i, lastseen, length, li, now, o, p, ref, size, t, toadd, update, url, x;
  update = (await R.get(UPDATE));
  now = Math.round(new Date() / 1000);
  if (update) {
    update = parseInt(update, 36);
    diff = WEEK - (now - update);
    if (diff > 0) {
      console.log(KEY + ' next update after ' + Math.round(1000 * diff / DAY) / 1000 + ' days');
      return;
    }
  }
  url = PROXY + 'https://raw.githubusercontent.com/7c/fakefilter/main/json/data.json';
  ({t, domains} = (await reqJson(url)));
  li = new Set();
  ref = Object.entries(domains);
  for (x of ref) {
    [host, o] = x;
    ({lastseen} = o);
    if ((t - lastseen) / 86400 < 365) {
      host = tld('http://' + host, {
        allowUnknownTLD: true
      });
      host = punycode.toUnicode(host.domain.trim().toLowerCase());
      if (host.length) {
        host = reverse(host);
        li.add(host);
      }
    }
  }
  li = [...li];
  li.sort();
  p = R.pipeline();
  toadd = [];
  fakefilter = (await R.getBuffer(FAKEFILTER));
  if (fakefilter) { // 删除过期的，不直接删除KEY，是防止有将来可以手工添加屏蔽域名
    fakefilter = new Set(unpack(fakefilter));
    for (i of li) {
      if (!fakefilter.delete(i)) {
        toadd.push(i);
      }
    }
    ({size} = fakefilter);
    if (fakefilter.size) {
      p.srem(KEY, ...fakefilter);
      console.log('remove', size);
    }
  } else {
    toadd = li;
  }
  ({length} = toadd);
  if (length) {
    p.sadd(KEY, ...toadd);
    console.log('add', length);
  }
  p.set(FAKEFILTER, pack(li));
  p.set(UPDATE, now.toString(36));
  p.exec();
};

if (process.argv[1] === decodeURI(new URL(import.meta.url).pathname)) {
  await main();
  process.exit();
}
