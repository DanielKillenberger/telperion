# fn-150 friction

## 2026-09-25: slim Wasm size stop

- Doing: measuring `telperion-field.wasm` after routing the slim crate through `pipeline::build`.
- Hindered: the brief says to stop if the fallback "pulls in much more code", but gives no threshold. The result was +48% (355,100 to 526,965 bytes), so I stopped for a host decision after R1 and the fix.
- Cost: about 5 minutes. The build itself is fast (about 10 s for the slim Wasm).
- Would remove it: a size budget in the spec, stated as bytes or percent, raw or gzip.
