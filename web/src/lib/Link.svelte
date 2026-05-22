<script lang="ts">
    import Focusable from "./Focusable.svelte";

    let { children, ...restProps } = $props();
</script>

<Focusable tag="a" {...restProps}>
    {@render children()}
</Focusable>
