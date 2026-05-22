<script lang="ts">
    import { browser } from "$app/environment";
    import LinkStyleButton from "$lib/LinkStyleButton.svelte";
    import FileDragAndDropArea from "./FileDragAndDropArea.svelte";
    import StatusMessage from "./StatusMessage.svelte";
    import Switch from "./Switch.svelte";
    import "@patternfly/elements/pf-spinner/pf-spinner.js";
    import "@patternfly/elements/pf-switch/pf-switch.js";

    let file: File | null = $state(null);
    let statusText = $state("");
    let statusVariant: "ok" | "ng" | "processing" = $state("ok");
    let usePointResample = $state(false);

    function saveToFile({
        buffer,
        length,
    }: {
        buffer: ArrayBuffer;
        length: number;
    }) {
        // save tempdecal.wad
        const blob = new Blob([new Uint8Array(buffer, 0, length)], {
            type: "application/octet-stream",
        });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.download = "tempdecal.wad";
        a.href = url;
        a.click();
        URL.revokeObjectURL(url);
    }

    const convert = (() => {
        if (!browser) {
            return () => {};
        }

        const worker = new Worker(new URL("./worker.ts", import.meta.url), {
            type: "module",
        });

        worker.onerror = (e: Event) => {
            if (!file) return;
            // console.error(e);
            statusVariant = "ng";
            statusText = `✗ Failed to convert ${file.name}.`;
        };

        worker.onmessage = (e: MessageEvent) => {
            if (!file) return;
            statusVariant = "ok";
            statusText = `✓ Conversion completed! (${file.name})`;
            saveToFile(e.data);
        };

        return async (f: File) => {
            file = f;
            statusVariant = "processing";
            statusText = `Processing ${file.name}...`;

            const img = await createImageBitmap(file);
            worker.postMessage({ img, usePointResample }, [img]);
        };
    })();
</script>

<Switch
    checked={usePointResample}
    onchange={(checked: boolean) => (usePointResample = checked)}
>
    Point resampling
    <small>
        (this will produce a crisp image especially for pixel art, not suitable
        for photographic images)
    </small>
</Switch>

<div class="converter-box">
    <FileDragAndDropArea accept="image/*" onchange={convert} ondrop={convert}>
        Drag and drop an image or
        <LinkStyleButton>Select an image</LinkStyleButton>
    </FileDragAndDropArea>
    <p>
        <small>
            The converter will run on your PC. No data uploading will happen.
            Tempdecal.wad will be saved into Download folder on your PC.
        </small>
    </p>
    <p>
        <StatusMessage variant={statusVariant}>{statusText}</StatusMessage>
    </p>
</div>

<style>
    .converter-box {
        margin: 0.62rem 2.62rem 0.38rem;
    }
</style>
