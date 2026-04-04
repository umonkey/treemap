<script lang="ts">
	import { handleShareTree } from '$lib/hooks/shareTree';
	import ShareIcon from '$lib/icons/ShareIcon.svelte';

	const { id } = $props<{ id: string }>();
</script>

<button type="button" onclick={() => handleShareTree(id)} title="Share this page">
	<ShareIcon />
</button>

<style>
	button {
		background: none;
		border: none;
		color: inherit;
		padding: 0;
		margin: 0;
		cursor: pointer;
	}
</style>
