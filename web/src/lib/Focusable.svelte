<script lang="ts">
    let { tag = "a", children, ...restProps } = $props();
</script>

<svelte:element
    this={tag}
    class={["link-style", restProps.class]}
    {...restProps}
>
    {@render children()}
</svelte:element>

<style>
    .link-style {
        background: none;
        border: none;
        padding: 0;
        font: inherit;
        text-decoration: underline;
        cursor: pointer;

        &:not(:visited) {
            color: #06c;
        }

        &:visited {
            color: #609;
        }

        @media (prefers-color-scheme: dark) {
            &:not(:visited) {
                color: rgb(158, 158, 255);
            }

            &:visited {
                color: rgb(208, 173, 240);
            }
        }
    }
</style>
