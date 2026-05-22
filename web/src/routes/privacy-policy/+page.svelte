<script lang="ts">
    import Link from "$lib/Link.svelte";
    import PageTitle from "$lib/PageTitle.svelte";
    import Panel from "$lib/Panel.svelte";
    import LinkStyleButton from "$lib/LinkStyleButton.svelte";
</script>

<PageTitle title="Privacy Policy" />

<header>
    <h1>Image Tempdecal Converter Privacy Policy</h1>
</header>
<main>
    <article>
        <header>
            <p>
                This privacy policy is effective as of August 27, 2022 (Last
                updated: May 22, 2026).
            </p>
        </header>
        <Panel header="Definitions">
            <div>
                <dl>
                    <dt>“Image Tempdecal Converter” / “we” / “us” / “our”</dt>
                    <dd>
                        The browser-based online tool that converts images into
                        tempdecal.wad files.
                    </dd>
                    <dt>“you” / “your”</dt>
                    <dd>Visitors to or users of our website.</dd>
                    <dt>“this policy” / “this privacy policy”</dt>
                    <dd>
                        This document, titled the “Image Tempdecal Converter
                        Privacy Policy”.
                    </dd>
                    <dt>“third party” / “third parties”</dt>
                    <dd>
                        Individuals, entities, websites, services, products, and
                        applications that are not controlled, managed, or
                        operated by us. This includes our hosting provider and
                        other service providers.
                    </dd>
                </dl>
            </div>
        </Panel>
        <Panel header="Policy Application">
            <p>
                This Policy applies to our collection and handling of
                information we receive from your use of our website. It also
                applies to information received from third parties.
            </p>
        </Panel>
        <Panel header="Information Collected">
            <div>
                <p>
                    We collect the following information through a third-party
                    service (<Link
                        href="https://www.cloudflare.com/web-analytics/"
                        >Cloudflare Web Analytics</Link
                    >):
                </p>
                <ul>
                    <li>Referrer</li>
                    <li>Pages visited</li>
                    <li>Operating system used</li>
                    <li>Web browser used</li>
                    <li>Device type used</li>
                    <li>Visits by country</li>
                    <li>Visits by time</li>
                </ul>
                <p>
                    This information is anonymized and aggregated, ensuring it
                    cannot be used to identify individual users.
                </p>
                <p>We do not collect the following information:</p>
                <ul>
                    <li>
                        Images you convert (all processing happens locally in
                        your browser)
                    </li>
                    <li>The number of times you visit our website</li>
                    <li>The purposes for which you use our service</li>
                </ul>
            </div>
        </Panel>
        <Panel header="Use of Collected Information">
            <p>
                We use the collected information to analyze website traffic,
                understand general usage trends, and improve our website's
                performance.
            </p>
        </Panel>
        <Panel header="Information Sharing and Security">
            <div>
                <p>
                    We keep your personal information secure and private. We
                    will not sell or rent your personal information to anyone.
                    We do not share or otherwise disclose your personal
                    information without your consent, except under specific
                    circumstances where disclosure is required by law.
                </p>
            </div>
        </Panel>
        <Panel header="Changes to This Privacy Policy">
            <p>
                We may update this Policy from time to time. We encourage you to
                review this Policy periodically. If we make material changes to
                this Policy, we will post a prominent notice on our homepage.
            </p>
        </Panel>
        <Panel header="Contact Information">
            <address>
                Owner: Nozomi Miyamori<br />
                Email: inspc43313@yahoo.co.jp
            </address>
        </Panel>
    </article>
</main>
<footer>
    <p>
        <LinkStyleButton onclick={() => history.back()}>Go Back</LinkStyleButton
        >
    </p>
</footer>
