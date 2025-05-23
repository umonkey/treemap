<script lang="ts">
	const { children } = $props();
</script>

<div class="buttons">
	{@render children()}
</div>

<style>
	.buttons {
		gap: var(--gap);
		display: flex;
		flex-direction: row;
	}
</style>
