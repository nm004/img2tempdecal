<script lang="ts">
	let { children } = $props();
</script>

{@render children()}

<style>
	:root {
		color-scheme: light dark;
		font-family: sans-serif;
		margin: 1.68em;
		line-height: 1.68;
		--pf-global--BackgroundColor--100: transparent;
	}
</style>
