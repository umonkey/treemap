<script>
    let { children } = $props();
</script>

<style>
:global {
    body {
        background-color: #14191f;
        color: rgb(241, 243, 245);
        font-family: InterVariable, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Liberation Sans", Helvetica, Arial, sans-serif;
    }

    a {
        color: rgb(32, 139, 254);
    }
}
</style>

{@render children()}
