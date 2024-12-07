<script>
    let { children } = $props();
</script>

<style>
:global {
    :root {
        --gap: 10px;
        font-size: 16px;

        /* Colors */
        --background-color: #fff;
        --icon-color-secondary: #6f869f;
        --link-color: #1083fe;
        --sep-color: #d4dbe2;
        --text-color-inactive: #42576c;
        --text-color: #0b0f14;
    }

    body {
        background-color: var(--background-color);
        color: var(--text-color);
        font-family: InterVariable, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Liberation Sans", Helvetica, Arial, sans-serif;
        padding: 0;
        margin: 0;
    }

    a {
        color: var(--link-color);
    }

    p {
        line-height: 1.5em;
    }
}

@media(prefers-color-scheme: dark) {
    :global {
        :root {
            --background-color: #14191f;
            --icon-color-secondary: #788ea5;
            --link-color: #208bfe;
            --sep-color: #2e4052;
            --text-color-inactive: #aebbc9;
            --text-color: #f1f3f5;
        }
    }
}
</style>

{@render children()}
