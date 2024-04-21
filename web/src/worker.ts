// This code is in the public domain.

import init, { convert } from './img2tempdecal_web.js';

const done_init = init();

globalThis.onmessage = async (ev: MessageEvent) => {
	await done_init;

	// buffer size is large enough to hold tempdecal.wad in memory.
	const buffer = new ArrayBuffer(20000);
	const length = convert(
		new Uint8Array(buffer),
		new Uint8Array(ev.data.buf),
		ev.data.width,
		ev.data.height,
		ev.data.use_point_resample,
	);

	globalThis.postMessage({ buffer, length }, [buffer]);
};
