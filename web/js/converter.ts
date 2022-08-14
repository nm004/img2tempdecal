export function convert_blob(
  imgSource: Blob | HTMLImageElement | ImageData | ImageBitmap,
  larger_size: boolean
) {
  return new Promise(async (resolve, reject) => {
    const imgBmp = await createImageBitmap(imgSource);
    const cv = document.createElement("canvas");
    cv.width = imgBmp.width;
    cv.height = imgBmp.height;

    const ctx = cv.getContext("2d");
    ctx.drawImage(imgBmp, 0, 0);

    const imgRaw = ctx.getImageData(0, 0, cv.width, cv.height);
    const buf = imgRaw.data.buffer;
    const width = imgRaw.width;
    const height = imgRaw.height;

    converterQ.push({ resolve, reject });
    converter.postMessage({ buf, width, height, larger_size }, [buf]);
  });
}

function onmessage(ev: MessageEvent) {
  // According to https://www.the303.org/tutorials/goldsrcspraylogo.html
  const MAX_TEXTURE_SIZE = 14336;

  // header, palette, padding, etc.
  const misc_size = 856;
  const bufsize =
    misc_size +
    MAX_TEXTURE_SIZE +
    MAX_TEXTURE_SIZE / 4 +
    MAX_TEXTURE_SIZE / 16 +
    MAX_TEXTURE_SIZE / 64;

  const buffer = new ArrayBuffer(bufsize);
  const length = wasm_bindgen.convert(
    new Uint8Array(ev.data.buf),
    ev.data.width,
    ev.data.height,
    ev.data.larger_size,
    new Uint8Array(buffer)
  );

  this.postMessage({ buffer, length }, [buffer]);
}

const converterScript = new Blob([
  `importScripts("${new URL(
    "../pkg/img2tempdecal_web.js",
    import.meta.url
  )}")\n`,
  `const init = async () => void await wasm_bindgen("${new URL(
    "../pkg/img2tempdecal_web_bg.wasm",
    import.meta.url
  )}")\n`,
  `init()\n`,
  `globalThis.onmessage = ${onmessage}\n`,
]);

const converterUrl = URL.createObjectURL(converterScript);
const converter = new Worker(converterUrl);
converter.onmessage = function (ev: MessageEvent) {
  const blob = new Blob(
    [new Uint8Array(ev.data.buffer).subarray(0, ev.data.length)],
    { type: "application/octet-stream" }
  );
  converterQ[0].resolve(blob);
  converterQ.shift();
};

converter.onerror = function (ev: ErrorEvent) {
  converterQ[0].reject();
  converterQ.shift();
};
const converterQ = [];
