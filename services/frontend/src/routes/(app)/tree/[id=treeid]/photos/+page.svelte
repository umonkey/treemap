<script lang="ts">
	import { getContext } from 'svelte';
	import Gallery from '$lib/components/photos/Gallery.svelte';
	import { TREE_CONTEXT_KEY, type TreeLayoutState } from '../layout.svelte.ts';

	const layoutState = getContext<TreeLayoutState>(TREE_CONTEXT_KEY);
</script>

{#if layoutState.tree}
	{@const tree = layoutState.tree}

	<Gallery id={tree.id} count={tree.files?.length} />
{/if}
