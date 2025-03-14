<script>
const { label, type = "submit", onClick, disabled = false } = $props();
const className = `button ${type}`;
</script>

<button type="button" disabled={!!disabled} class={className} onclick={onClick}>{label}</button>
