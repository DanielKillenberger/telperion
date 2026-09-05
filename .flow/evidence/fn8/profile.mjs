const {chromium}=await import(process.env.PLAYWRIGHT_MODULE||'playwright');
import {writeFile} from 'node:fs/promises';
const url=process.env.BROWSER_URL||'http://127.0.0.1:5185';
const browser=await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE||'/usr/bin/chromium',headless:true,args:['--no-sandbox']});
try {
const page=await browser.newPage();await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<!doctype html>'}));await page.goto(url);
await page.evaluate(async()=>{const{TreeEngine,TELPERION}=await import('/src/browser/core.ts');window.e=await TreeEngine.create();window.f=TELPERION;window.e.build(window.f,{surface:true,foliage:true});window.e.release();});
const cdp=await page.context().newCDPSession(page);await cdp.send('Profiler.enable');await cdp.send('Profiler.start');
await page.evaluate(()=>{window.e.build(window.f,{surface:true,foliage:true});window.e.release();});
const result=await cdp.send('Profiler.stop');await writeFile(process.env.PROFILE_OUTPUT||'/tmp/fn8-wasm-profile.json',JSON.stringify(result));
} finally {await browser.close();}
