<script lang="ts">
    import "@patternfly/elements/pf-panel/pf-panel.js";
    let { header, children } = $props();
</script>

<pf-panel>
    <h3 slot="header">{header}</h3>
    {@render children()}
</pf-panel>
