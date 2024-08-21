/*
 * This file is a part of img2tempdecal by Nozomi Miyamori.
 * img2tempdecal is distributed under the MIT-0 license and the Public Domain.
 */

import init, { convert } from './img2tempdecal_web.js';

(async () => { await init() })();

const pointResampleSwitch = document.getElementById('point-resample-switch') as HTMLInputElement;
const imgDropzone = document.getElementById('img-dropzone') as HTMLElement;
const imgInput = document.getElementById('img-input') as HTMLInputElement;
const statusMsg = document.getElementById('status-msg') as HTMLElement;

const handleConvertRequest = (file: File) => {

	statusMsg.className = '';
	statusMsg.innerHTML = '';
	const fileName = document.createElement('samp');
	fileName.textContent = file.name;

	const spinner = document.createElement('pf-spinner');
	spinner.setAttribute('size', 'md');
	statusMsg.className = '';
	statusMsg.innerHTML = `${spinner.outerHTML} Processing ${fileName.outerHTML}...`;

	// buffer size is large enough to hold tempdecal.wad in memory.
	const buffer = new Uint8Array(new ArrayBuffer(20000));

	createImageBitmap(file).then((imgBmp: ImageBitmap) => {
		const canvas = new OffscreenCanvas(imgBmp.width, imgBmp.height);
		const ctx = canvas.getContext('2d');
		if (!ctx) {
			throw new Error('Failed to get canvas context');
		}
		ctx.drawImage(imgBmp, 0, 0);
		const imgRaw = ctx.getImageData(
			0, 0, imgBmp.width, imgBmp.height, { colorSpace: 'srgb'});
		return convert(buffer, new Uint8Array(imgRaw.data.buffer),
			imgRaw.width, imgRaw.height, pointResampleSwitch.checked)
	}).then((length: number) => {
		statusMsg.className = 'ok';
		statusMsg.innerHTML = `✓ Conversion completed! (${fileName.outerHTML})`;

		// save tempdecal.wad
		const a = document.createElement('a');
		a.download = 'tempdecal.wad';
		a.href = URL.createObjectURL(new Blob(
			[buffer.subarray(0, length)],
			{ type: 'application/octet-stream' }
		));
		a.click();
		URL.revokeObjectURL(a.href);
	}).catch(e => {
		console.error(e);
		statusMsg.className = 'ng'
		statusMsg.innerHTML = `✗ Failed to convert ${fileName.outerHTML}, sorry.`;
		return;
	});

};

window.addEventListener('dragover', (ev: Event) => {
	ev.preventDefault();
	ev.stopPropagation();
}, {capture: true});

window.addEventListener('drop', (ev: Event) => {
	ev.preventDefault();
}, {capture: true});

imgInput.onchange = () => {
	const f = imgInput.files?.item(0);
	f && handleConvertRequest(f);
};

imgDropzone.ondragenter = () => {
	imgDropzone.className = 'file-dragging';
};

imgDropzone.ondragleave = () => {
	imgDropzone.className = '';
};

imgDropzone.ondrop = (ev: Event) => {
	ev.preventDefault();
	imgDropzone.className = '';
	const f = (ev as DragEvent).dataTransfer?.files[0];
	f && handleConvertRequest(f);
};

imgDropzone.onclick = () => {
	imgInput.click();
};
