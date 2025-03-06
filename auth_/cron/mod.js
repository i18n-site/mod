#!/usr/bin/env -S deno run -A

import curl from "@8v/curl";

const URL_FAKEFILTER = 'https://raw.githubusercontent.com/7c/fakefilter/main/json/data.json'
console.log(await curl(URL_FAKEFILTER))
