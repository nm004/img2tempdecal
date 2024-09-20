import init, { convert } from './img2tempdecal_web.js';

(async () => { await init({}); })();

self.onmessage = (e: MessageEvent) => {
	const canvas = new OffscreenCanvas(e.data.imgBmp.width, e.data.imgBmp.height);
	const ctx = canvas.getContext('2d');
	if (!ctx) {
		throw new Error('Failed to get canvas context');
	}
	ctx.drawImage(e.data.imgBmp, 0, 0);
	const imgRaw = ctx.getImageData(
		0, 0, canvas.width, canvas.height, { colorSpace: 'srgb'});

	// buffer size is large enough to hold tempdecal.wad in memory.
	const buffer = new ArrayBuffer(0x7FFF);
	const length = convert(new Uint8Array(buffer), new Uint8Array(imgRaw.data.buffer),
		imgRaw.width, imgRaw.height, e.data.usePointResample)

	self.postMessage({ buffer, length }, [ buffer ]);
}
