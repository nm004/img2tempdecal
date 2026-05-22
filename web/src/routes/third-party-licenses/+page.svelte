<script lang="ts">
    import PageTitle from "$lib/PageTitle.svelte";
    import LinkStyleButton from "$lib/LinkStyleButton.svelte";
    import licenses from "$lib/assets/licenses?raw";
</script>

<PageTitle title="Licenses" />

<header>
    <h1>Third party licenses</h1>
</header>
<main>
    <p>
        Image Tempdecal Converter is built upon the following open-source
        software. We would like to thank all the creators who made these tools
        freely available.
    </p>
    <pre>{licenses}</pre>
</main>

<footer>
    <p>
        <LinkStyleButton onclick={() => history.back()}>Go back</LinkStyleButton
        >
    </p>
</footer>
