// This code is in the public domain.

const pointResampleSwitch = document.getElementById('point-resample-switch') as HTMLInputElement;
const imgDropzone = document.getElementById('img-dropzone') as HTMLElement;
const imgInput = document.getElementById('img-input') as HTMLInputElement;
const statusMsg = document.getElementById('status-msg') as HTMLElement;

const convert = async (
	imgSource: Blob | HTMLImageElement | ImageData | ImageBitmap,
	use_point_resample: boolean
): Promise<Blob> => {
	const imgBmp = await createImageBitmap(imgSource);
	const cv = document.createElement('canvas');
	cv.width = imgBmp.width;
	cv.height = imgBmp.height;

	const ctx = cv.getContext('2d');
	if (ctx === null) {
		throw new Error('Failed to get canvas context');
	}

	ctx.drawImage(imgBmp, 0, 0);
	const imgRaw = ctx.getImageData(0, 0, cv.width, cv.height);
	const msg = {
		buf: imgRaw.data.buffer,
		width: imgRaw.width,
		height: imgRaw.height,
		use_point_resample,
	}
	const p = new Promise<Blob>((resolve, reject) =>
		converterQ.push({ resolve, reject })
	);
	converter.postMessage(msg, [msg.buf]);
	return p;
};

const converter = new Worker(new URL('./worker', import.meta.url), {
	type: 'module'
});
const converterQ: { resolve(blob: Blob): void, reject(ev: ErrorEvent): void }[] = [];
converter.addEventListener('message', (ev: MessageEvent) => {
	const blob = new Blob(
		[new Uint8Array(ev.data.buffer).subarray(0, ev.data.length)],
		{ type: 'application/octet-stream' }
	);
	converterQ[0].resolve(blob);
	converterQ.shift();
});
converter.addEventListener('error', (ev: ErrorEvent) => {
	converterQ[0].reject(ev);
	converterQ.shift();
});

const handle_convert_request = async (file: File) => {
	set_status_msg('', '');
	const fn = document.createElement('samp');
	fn.textContent = file.name;

	const spinner = document.createElement('pf-spinner');
	spinner.setAttribute('size', 'md');
	set_status_msg('', `${spinner.outerHTML} Processing ${fn.outerHTML}...`);

	let blob;
	try {
		blob = await convert(file, pointResampleSwitch.checked);
	} catch (e) {
		console.error(e);
		set_status_msg('ng', `✗ <strong>Oops</strong>, failed to convert ${fn.outerHTML}, sorry.`);
		return;
	}
	set_status_msg('ok', `✓ Conversion completed! (${fn.outerHTML})`);

	// save tempdecal.wad
	const a = document.createElement('a');
	a.download = 'tempdecal.wad';
	a.href = URL.createObjectURL(blob);
	a.click();
	URL.revokeObjectURL(a.href);
};

const set_status_msg = (msg_class: '' | 'ok' | 'ng', inner_html: string) => {
	statusMsg.className = msg_class;
	statusMsg.innerHTML = inner_html;
}

window.addEventListener('dragover', (ev: Event) => {
	ev.preventDefault();
	ev.stopPropagation();
}, {capture: true});

window.addEventListener('drop', (ev: Event) => {
	ev.preventDefault();
}, {capture: true});

imgInput.addEventListener('change', () => {
	const f = imgInput.files?.item(0);
	f && handle_convert_request(f);
});

imgDropzone.addEventListener('dragenter', (() => {
	imgDropzone.className = 'file-dragging';
}));

imgDropzone.addEventListener('dragleave', (() => {
	imgDropzone.className = '';
}));

imgDropzone.addEventListener('drop', ((ev: Event) => {
	ev.preventDefault();
	imgDropzone.className = '';
	const f = (ev as DragEvent).dataTransfer?.files[0];
	f && handle_convert_request(f);
}));

imgDropzone.addEventListener('click', () => {
	imgInput.click();
});

