<script lang="ts">
    import '@patternfly/elements/pf-v5-switch/pf-v5-switch.js';
    const id = $props.id();

    let { children, checked, onchange } = $props();
</script>

<form>
    <pf-v5-switch
        {id}
        show-check-icon
        {checked}
        onchange={(e: Event) => {
            onchange((e.target as HTMLInputElement).checked);
        }}
    ></pf-v5-switch>
    <label for={id}>{@render children()}</label>
</form>

<style>
    form {
        cursor: pointer;

        & > label {
            cursor: pointer;
            padding-inline: 0.38ex;
        }
    }
</style>
