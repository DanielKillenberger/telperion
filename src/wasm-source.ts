/* A Wasm module that ships beside the JavaScript that loads it. Each call
 * site names its file as a literal URL against `import.meta.url`, the form a
 * consumer's bundler reads as an asset and emits into its own build. In a
 * browser the URL is fetched. In Node, `import.meta.url` is a
 * `file:` URL that `fetch` refuses, so the file is read from disk through an
 * import a browser bundler is told to leave alone. */

interface FileReader { readFile(url: URL): Promise<Uint8Array> }

export async function wasmSource(url: URL): Promise<Response | Uint8Array> {
  if (url.protocol !== "file:") return fetch(url);
  const specifier = "node:fs/promises";
  const { readFile } = (await import(/* @vite-ignore */ /* webpackIgnore: true */ specifier)) as FileReader;
  return readFile(url);
}
