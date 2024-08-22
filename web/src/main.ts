/*
 * This file is a part of img2tempdecal by Nozomi Miyamori.
 * img2tempdecal is distributed under the MIT-0 license and the Public Domain.
 */

document.addEventListener('DOMContentLoaded', e => {
	const pointResampleSwitch = document.getElementById('point-resample-switch') as HTMLInputElement;
	const statusMsg = document.getElementById('status-msg') as HTMLElement;

	const worker = new Worker(new URL('./worker', import.meta.url), { type: 'module' });
	const handleConvertRequest = (file: File) => {
		statusMsg.className = '';
		statusMsg.innerHTML = '';
		const fileName = document.createElement('samp');
		fileName.textContent = file.name;

		const spinner = document.createElement('pf-spinner');
		spinner.setAttribute('size', 'md');
		statusMsg.className = '';
		statusMsg.innerHTML = `${spinner.outerHTML} Processing ${fileName.outerHTML}...`;

		const onError = (e: Event) => {
			worker.removeEventListener('message', onMessage);
			console.error(e);
			statusMsg.className = 'ng'
			statusMsg.innerHTML = `✗ Failed to convert ${fileName.outerHTML}, sorry.`;
		};

		const onMessage = (e: Event) => {
			worker.removeEventListener('error', onError);
			statusMsg.className = 'ok';
			statusMsg.innerHTML = `✓ Conversion completed! (${fileName.outerHTML})`;
			// save tempdecal.wad
			const { buffer, length } = (e as MessageEvent).data;
			const a = document.createElement('a');
			a.download = 'tempdecal.wad';
			a.href = URL.createObjectURL(new Blob(
				[new Uint8Array(buffer, 0, length)],
				{ type: 'application/octet-stream' }
			));
			a.click();
			URL.revokeObjectURL(a.href);
		};

		createImageBitmap(file).then((imgBmp: ImageBitmap) => {
			const canvas = new OffscreenCanvas(imgBmp.width, imgBmp.height);
			const ctx = canvas.getContext('2d');
			if (!ctx) {
				throw new Error('Failed to get canvas context');
			}
			ctx.drawImage(imgBmp, 0, 0);
			const imgRaw = ctx.getImageData(
				0, 0, imgBmp.width, imgBmp.height, { colorSpace: 'srgb'});
			const msg = {
				buffer: imgRaw.data.buffer,
				width: imgRaw.width,
				height: imgBmp.height,
				usePointResample: pointResampleSwitch.checked,
			};
			worker.addEventListener('message', onMessage, { once: true });
			worker.addEventListener('error', onError, { once: true });
			worker.postMessage(msg, [imgRaw.data.buffer])
		}).catch(onError);
	};

	window.addEventListener('dragover', (ev: Event) => {
		ev.preventDefault();
		ev.stopPropagation();
	}, {capture: true});

	window.addEventListener('drop', (ev: Event) => {
		ev.preventDefault();
	}, {capture: true});

	const imgInput = document.getElementById('img-input') as HTMLInputElement;
	imgInput.onchange = () => {
		const f = imgInput.files?.item(0);
		f && handleConvertRequest(f);
	};

	const imgDropzone = document.getElementById('img-dropzone') as HTMLElement;
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
}, { once: true });
