<script lang="ts">
    import PageTitle from "$lib/PageTitle.svelte";
    import Link from "$lib/Link.svelte";
    import Panel from "$lib/Panel.svelte";
    import Converter from "./Converter.svelte";

    function handleDragOver(ev: DragEvent) {
        ev.preventDefault();
    }

    function handleDropWindow(ev: DragEvent) {
        ev.preventDefault();
    }
</script>

<PageTitle />
<svelte:window ondragover={handleDragOver} ondrop={handleDropWindow} />

<header>
    <h1>Image Tempdecal Converter</h1>
</header>

<main>
    <Converter />

    <h2>Q&amp;A</h2>

    <Panel header="What is tempdecal.wad?">
        <p>
            <code>tempdecal.wad</code> is a custom spray format used in
            <cite
                ><Link
                    rel="external"
                    target="_blank"
                    href="https://en.wikipedia.org/wiki/GoldSrc">GoldSrc</Link
                ></cite
            >
            engine games such as
            <cite
                ><Link
                    rel="external"
                    target="_blank"
                    href="https://store.steampowered.com/app/70/HalfLife/"
                    >Half-Life</Link
                ></cite
            >,
            <cite
                ><Link
                    rel="external"
                    target="_blank"
                    href="https://store.steampowered.com/app/10/CounterStrike/"
                    >Counter-Strike 1.6</Link
                ></cite
            >, and
            <cite
                ><Link
                    rel="external"
                    target="_blank"
                    href="https://store.steampowered.com/app/225840/Sven_Coop/"
                    >Sven Co-op</Link
                ></cite
            >. These games only offer a limited selection of preset sprays by
            default. You can use any custom image as a spray with a custom
            <code>tempdecal.wad</code>, including full-color images!
        </p>
    </Panel>

    <Panel header="Where should I place the tempdecal.wad?">
        <p>
            The default installation paths for each supported game are listed
            below. Save your <code>tempdecal.wad</code> into the appropriate folder:
        </p>
        <dl>
            <dt>Sven Co-op</dt>
            <dd><code>&lt;Game Folder&gt;\svencoop</code></dd>
            <dt>Counter-Strike 1.6</dt>
            <dd><code>&lt;Game Folder&gt;\cstrike</code></dd>
            <dt>Half-Life</dt>
            <dd><code>&lt;Game Folder&gt;\valve</code></dd>
        </dl>
        <p>
            To locate the <strong>Game Folder</strong>, open Steam, navigate to
            your <strong>Library</strong>, right-click the game, and select
            <strong>Manage &gt; Browse local files</strong>.
        </p>
        <p>
            <strong>Additionally</strong>, we highly recommend setting the
            <code>tempdecal.wad</code> to read-only to prevent the game from
            overwriting your custom spray with a default one. To do this,
            right-click the file, select
            <strong>Properties</strong>, check the <strong>Read-only</strong>
            box under the General tab, and click <strong>OK</strong>.
        </p>
        <p>That's it! You are now ready to use your custom spray in-game.</p>
        <section>
            <h4>Note for Sven Co-op players</h4>
            <p>
                You must enable custom sprays in the game options. Launch the
                game, open the <strong>Options</strong> menu, navigate to the
                <strong>Multiplayer</strong>
                tab, and click <strong>Advanced...</strong>. Under the
                <strong>Graphics</strong>
                section, set <strong>Custom player sprays</strong> to
                <strong>Enabled</strong>. We also recommend setting the cache
                size to <strong>Large cache (16MB)</strong>.
            </p>
        </section>
        <section>
            <h4>Note for pixel art sprays</h4>
            <p>
                To ensure pixel art sprays remain sharp and crisp instead of
                blurry, open the in-game developer console and run the following
                command: <code>gl_texturemode GL_NEAREST_MIPMAP_LINEAR</code>.
            </p>
        </section>
    </Panel>

    <Panel header="What image formats does this tool support?">
        <p>The tool supports the following image formats:</p>
        <ul>
            <li>JPEG</li>
            <li>PNG</li>
            <li>GIF</li>
            <li>BMP</li>
            <li>WebP</li>
            <li>AVIF</li>
        </ul>
        <p>
            In general, any image format supported by your web browser can be
            converted, with the exception of SVG.
        </p>
    </Panel>

    <Panel header="Is this tool free?">
        <p>Yes, absolutely! It is completely free to use and open source.</p>
    </Panel>

    <Panel header="Is the source code available?">
        <p>
            Yes, you can access the full source code on GitHub: <Link
                rel="external"
                href="https://github.com/nm004/img2tempdecal"
                target="_blank">nm004/img2tempdecal</Link
            >.
        </p>
    </Panel>
</main>

<footer>
    <p>
        <Link href="./privacy-policy">Privacy policy</Link>
        <br />
        <Link href="./third-party-licenses">Third party licenses</Link>
        <br />
        <Link href="./">Back to top</Link>
    </p>
</footer>

<style>
    dt {
        padding-block-start: 0.38rem;
    }
</style>
