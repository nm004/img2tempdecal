<script lang="ts">
    let { accept, children, onchange, ondrop } = $props();
    const inputId = $props.id();
    let dragging = $state(false);
    let inputElement: HTMLInputElement;
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<form
    class={{ dragging }}
    ondragenter={() => {
        dragging = true;
    }}
    ondragleave={() => {
        dragging = false;
    }}
    ondragover={(ev) => {
        ev.preventDefault();
    }}
    onclick={() => {
        inputElement.click();
    }}
    ondrop={(ev) => {
        ev.preventDefault();
        dragging = false;
        const f = ev.dataTransfer?.files?.[0];
        f && ondrop(f);
    }}
>
    <input
        type="file"
        id={inputId}
        {accept}
        bind:this={inputElement}
        onchange={(e) => {
            const t = e.currentTarget;
            const f = t.files?.[0];
            f && onchange(f);
            t.value = "";
        }}
    />
    <label for={inputId}>
        {@render children()}
    </label>
</form>

<style>
    form {
        font-size: larger;
        padding-block: 6.85rem;
        border: 2px dashed rgb(34, 34, 34, 0.62);
        border-radius: 11px;
        text-align: center;

        @media (prefers-color-scheme: dark) {
            border-color: rgb(235, 235, 235, 0.62);
        }

        &,
        & > * {
            cursor: pointer;
        }

        &:hover,
        &.dragging {
            border-style: solid;
        }

        &.dragging {
            background: rgb(2, 101, 220, 0.1);
            border-color: rgb(20, 122, 243);
        }

        & > input[type="file"] {
            display: none;
        }
    }
</style>
