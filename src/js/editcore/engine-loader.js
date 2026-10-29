import createModule from 'bentopdf-pdfium';
import embedPdfWasmUrl from 'bentopdf-pdfium/editcore.wasm?url';

const inBrowser = typeof window !== 'undefined';

let ENGINE_VERSION = 'dev';
try {
  ENGINE_VERSION = __ENGINE_VERSION__;
} catch {
  ENGINE_VERSION = 'dev';
}

function resolveWasmUrl() {
  if (inBrowser) {
    const base = window.location.origin || window.location.href;
    const url = new URL(embedPdfWasmUrl, base);
    if (import.meta.env?.DEV) {
      url.searchParams.set('v', ENGINE_VERSION);
    }
    return url.href;
  }
  const resolve = import.meta.resolve;
  if (typeof resolve !== 'function') return null;
  return new URL(resolve('bentopdf-pdfium/editcore.wasm')).pathname;
}

const wasmUrl = resolveWasmUrl();

export const ENGINE_BUILD = `bentopdf-pdfium@${ENGINE_VERSION}`;

export function createEngineModule(options) {
  return createModule({
    ...(options ?? {}),
    locateFile: (file, prefix) =>
      file.endsWith('.wasm')
        ? (wasmUrl ?? `${prefix}editcore.wasm`)
        : `${prefix}${file}`,
  });
}
