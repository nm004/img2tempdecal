<script lang="ts">
    let { title = "" } = $props();
    const siteName = "Image Tempdecal Converter";
</script>

<svelte:head>
    <title>{title ? `${title} | ${siteName}` : siteName}</title>
</svelte:head>
