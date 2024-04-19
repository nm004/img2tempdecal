// This code is in the public domain.

import init, { convert } from './img2tempdecal_web.js';

const done_init = init();

globalThis.onmessage = async (ev: MessageEvent) => {
	await done_init;

	// Ref. https://www.the303.org/tutorials/goldsrcspraylogo.html
	const MAX_TEXTURE_SIZE = 14336;

	// header, palette, padding, etc.
	const misc_size = 856;
	const bufsize = misc_size
		  + MAX_TEXTURE_SIZE
		  + MAX_TEXTURE_SIZE / 4
		  + MAX_TEXTURE_SIZE / 16
		  + MAX_TEXTURE_SIZE / 64;

	const buffer = new ArrayBuffer(bufsize);
	const length = convert(
		new Uint8Array(buffer),
		new Uint8Array(ev.data.buf),
		ev.data.width,
		ev.data.height,
		ev.data.use_point_resample,
	);

	globalThis.postMessage({ buffer, length }, [buffer]);
};
