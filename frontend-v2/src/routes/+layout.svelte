<script>
    let { children } = $props();
</script>

<style>
:global {
    :root {
        --background-color: #14191f;
        --text-color: #f1f3f5;
        --link-color: #208bfe;
        --sep-color: #2e4052;
        --gap: 10px;
        font-size: 16px;
    }

    body {
        background-color: var(--background-color);
        color: var(--text-color);

        font-family: InterVariable, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Liberation Sans", Helvetica, Arial, sans-serif;
    }

    a {
        color: var(--link-color);
    }
}
</style>

{@render children()}
