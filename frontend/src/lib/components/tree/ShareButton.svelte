<script lang="ts">
	import { ShareIcon } from '$lib/icons';
	import { handleShareTree } from '$lib/hooks';

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
