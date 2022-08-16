import { Toast, ToastVariants } from "@spectrum-web-components/toast";
import { convert_blob } from "./js/converter";
import { Switch } from "@spectrum-web-components/switch";

const imgDropzone = document.getElementById("img-dropzone");
const imgInput = document.getElementById("img-input") as HTMLInputElement;
const statusArea = document.getElementById("status-area") as HTMLSelectElement;
const largerSizeSwitch = document.getElementById(
  "larger-size-switch"
) as Switch;

function push_status(variant: ToastVariants, text: string) {
  const output = document.createElement("output");

  output.textContent = text;
  const toast = new Toast();
  toast.open = true;
  toast.variant = variant;
  toast.timeout = 10000;
  toast.appendChild(output);
  toast.addEventListener("close", function () {
    this.remove();
  });

  statusArea.appendChild(toast);
}

function do_convert(file: File) {
  push_status("info", "In progress...");
  convert_blob(file, largerSizeSwitch.checked)
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

imgInput.addEventListener("change", function () {
  const f = this.files.item(0);
  if (!f) {
    return;
  }
  do_convert(this.files[0]);
});

imgDropzone.addEventListener("sp-dropzone-drop", function (e: CustomEvent) {
  const f = e.detail.dataTransfer.items[0].getAsFile();
  if (!f) {
    return;
  }

  do_convert(f);
});

imgDropzone.addEventListener("click", function () {
  imgInput.click();
});
