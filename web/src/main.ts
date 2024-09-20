/*
 * This file is a part of img2tempdecal by Nozomi Miyamori.
 * img2tempdecal is distributed under the MIT-0 license and the Public Domain.
 */

document.addEventListener('DOMContentLoaded', e => {
	const pointResampleSwitch = document.getElementById('point-resample-switch') as HTMLInputElement;
	const statusMsg = document.getElementById('status-msg') as HTMLElement;

	const worker = new Worker(new URL('./worker', import.meta.url), { type: 'module' });

	let fileName: string;

	worker.onerror = (e: Event) => {
		console.error(e);
		statusMsg.className = 'ng'
		statusMsg.innerHTML = `✗ Failed to convert ${fileName}, sorry.`;
	};

	worker.onmessage = (e: Event) => {
		statusMsg.className = 'ok';
		statusMsg.innerHTML = `✓ Conversion completed! (${fileName})`;

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

	const handleConvertRequest = (file: File) => {
		const spinner = document.createElement('pf-spinner');
		spinner.setAttribute('size', 'md');
		statusMsg.className = '';
		statusMsg.innerHTML = `${spinner.outerHTML} Processing ${file.name}...`;
		fileName = file.name;

		createImageBitmap(file).then((imgBmp: ImageBitmap) => {
			worker.postMessage({
				imgBmp,
				usePointResample: pointResampleSwitch.checked
			}, [imgBmp]);

		});
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
