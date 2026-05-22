<script lang="ts">
    import "@patternfly/elements/pf-spinner/pf-spinner.js";
    let {
        children,
        variant,
    }: { children: any; variant: "ok" | "ng" | "processing" } = $props();
</script>

<span class={variant}>
    {#if variant === "processing"}
        <pf-spinner size="md"></pf-spinner>
    {/if}
    {@render children()}
</span>

<style>
    .ok {
        color: green;
    }

    .ng {
        color: crimson;
    }

    @media (prefers-color-scheme: dark) {
        .ok {
            color: lightgreen;
        }

        .ng {
            color: indianred;
        }
    }
</style>
