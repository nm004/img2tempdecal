import init, { convert } from './img2tempdecal_web.js';
const init_ = init();

self.onmessage = (e: MessageEvent) => {
	// buffer size is large enough to hold tempdecal.wad in memory.
	const buffer = new ArrayBuffer(0x7FFF);
	const length = convert(new Uint8Array(buffer), new Uint8Array(e.data.buffer),
		e.data.width, e.data.height, e.data.usePointResample)

	self.postMessage({ buffer, length }, [ buffer ]);
}

(async () => await init_)();
