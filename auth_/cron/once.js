#!/usr/bin/env bun

import {exit} from "node:process"

let RUNNING = 0

const {resolve, promise} = Promise.withResolvers()

Deno.cron = async (kind, cron, func)=>{
  ++RUNNING
  try {
    await func()
  } catch (e) {
    console.error(e)
  } finally {
    --RUNNING
    console.log('running', RUNNING)
    if(!RUNNING){
      resolve()
    }
  }
  return 
}

await import('./main.js')
await promise
exit()
