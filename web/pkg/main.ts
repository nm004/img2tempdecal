import { Toast, ToastVariants } from "@spectrum-web-components/toast";
import { Switch } from "@spectrum-web-components/switch";
import { Dropzone } from "@spectrum-web-components/dropzone";

const imgDropzone = document.getElementById("img-dropzone") as Dropzone;
const imgInput = document.getElementById("img-input") as HTMLInputElement;
const statusArea = document.getElementById("status-area") as HTMLSelectElement;
const largerSizeSwitch = document.getElementById("larger-size-switch") as Switch;
const pointResampleSwitch = document.getElementById("point-resample-switch") as Switch;

const push_status = (variant: ToastVariants, text: string) => {
	const output = document.createElement("output");

	output.textContent = text;
	const toast = new Toast();
	toast.open = true;
	toast.variant = variant;
	toast.timeout = 10000;
	toast.appendChild(output);
	toast.addEventListener("close", () => {
		toast.remove();
	});

	statusArea.appendChild(toast);
}

const do_convert = (file: File) => {
	push_status("info", "Processing...");
	convert_blob(file, largerSizeSwitch.checked, pointResampleSwitch.checked)
		.then((blob: Blob) => {
			// save wad file
			const a = document.createElement("a");
			a.download = "tempdecal.wad";
			a.href = URL.createObjectURL(blob);
			a.click();
			URL.revokeObjectURL(a.href);

			push_status("positive", "Done!");
		})
		.catch(() => {
			push_status("negative", "Oops, couldn't convert your file.");
		});
}

imgInput.addEventListener("change", () => {
	if (imgInput.files === null) return;
	const f = imgInput.files.item(0);
	if (!f) {
		return;
	}
	do_convert(f);
});


imgDropzone.addEventListener("sp-dropzone-drop", ((ev: DragEvent) => {
	if (ev.dataTransfer === null) return;
	const f = ev.dataTransfer.items[0].getAsFile();
	if (!f) {
		return;
	}

	do_convert(f);
}) as EventListener);

imgDropzone.addEventListener("click", () => {
	imgInput.click();
});

export const convert_blob = (
	imgSource: Blob | HTMLImageElement | ImageData | ImageBitmap,
	larger_size: boolean,
	use_point_resample: boolean
): Promise<Blob> => {
	return new Promise(async (resolve, reject) => {
		const imgBmp = await createImageBitmap(imgSource);
		const cv = document.createElement("canvas");
		cv.width = imgBmp.width;
		cv.height = imgBmp.height;

		const ctx = cv.getContext("2d");
		if (ctx === null) return;
		ctx.drawImage(imgBmp, 0, 0);

		const imgRaw = ctx.getImageData(0, 0, cv.width, cv.height);
		const buf = imgRaw.data.buffer;
		const width = imgRaw.width;
		const height = imgRaw.height;

		converterQ.push({ resolve, reject });
		converter.postMessage({ buf, width, height, larger_size, use_point_resample }, [buf]);
	});
}

const converter = new Worker(new URL('./worker', import.meta.url), {
	type: 'module'
});

const converterQ: { resolve(blob: Blob): void, reject(ev: ErrorEvent): void }[] = [];
converter.onmessage = (ev: MessageEvent) => {
	const blob = new Blob(
		[new Uint8Array(ev.data.buffer).subarray(0, ev.data.length)],
		{ type: "application/octet-stream" }
	);
	converterQ[0].resolve(blob);
	converterQ.shift();
};

converter.onerror = (ev: ErrorEvent) => {
	converterQ[0].reject(ev);
	converterQ.shift();
};
