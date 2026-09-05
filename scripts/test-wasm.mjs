import { readFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { pathToFileURL } from 'node:url';
import assert from 'node:assert/strict';
const { chromium } = await import(process.env.PLAYWRIGHT_MODULE ? pathToFileURL(process.env.PLAYWRIGHT_MODULE).href : 'playwright');
execFileSync('cargo', ['build','--release','--target','wasm32-unknown-unknown','-p','telperion-wasm'], {stdio:'inherit',timeout:600_000});
const native = JSON.parse(execFileSync('cargo',['run','--quiet','--release','-p','telperion-core','--example','foundation'],{encoding:'utf8',timeout:600_000}));
const bytes = [...await readFile(new URL('../target/wasm32-unknown-unknown/release/telperion_wasm.wasm',import.meta.url))];
const browser = await chromium.launch({headless:true,...(process.env.CHROMIUM_EXECUTABLE ? {executablePath:process.env.CHROMIUM_EXECUTABLE} : {})});
try {
  const page = await browser.newPage();
  const proof = await page.evaluate(async ({bytes,native}) => {
    const check=(condition,message)=>{if(!condition) throw Error(message);};
    const {instance}=await WebAssembly.instantiate(new Uint8Array(bytes));
    const e=instance.exports;
    const sample=(seed=42,count=64,height=24)=>e.sample_envelope(seed,count,height,0.3,0.3,0.45,2.2);
    const copy=()=>Array.from(new Float64Array(e.memory.buffer,e.output_ptr(),e.output_len()).slice());
    check(sample()===0,'valid input');
    const owned=copy();const saved=JSON.stringify(owned);check(owned.length===192,'nonempty output');
    let maxError=0;for(let i=0;i<owned.length;i++) maxError=Math.max(maxError,Math.abs(owned[i]-native[i]));
    check(maxError<=1e-11,'native/Wasm mismatch');
    e.release();check(e.output_len()===0,'release');e.release();
    check(sample(7,1000)===0,'reuse/grow memory');
    check(JSON.stringify(owned)===saved,'copied result changed');
    check(sample()===0&&JSON.stringify(copy())===JSON.stringify(owned),'determinism after reuse');
    check(sample(42,1,NaN)===1&&e.output_len()===0,'NaN rejection clears stale output');
    check(sample(42,1,Infinity)===1,'infinite input');
    check(sample(42,1,-1)===1,'negative input');
    check(sample(42,1_000_001)===2&&e.output_len()===0,'resource bound');
    check(sample(42,0)===0&&e.output_len()===0,'valid empty');
    check(sample(42,10,0)===0&&e.output_len()===0,'degenerate');
    e.release();
    return {values:owned.length,maxError,ownership:'copied; release idempotent; reused; malformed clears output'};
  },{bytes,native});
  assert.equal(proof.values,native.length);
  console.log(JSON.stringify(proof,null,2));
} finally {await browser.close();}
