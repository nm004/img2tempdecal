<script lang="ts">
    import '@patternfly/elements/pf-v5-panel/pf-v5-panel.js';
    let { header, children } = $props();
</script>

<pf-v5-panel>
    <h3 slot="header">{header}</h3>
    {@render children()}
</pf-v5-panel>
