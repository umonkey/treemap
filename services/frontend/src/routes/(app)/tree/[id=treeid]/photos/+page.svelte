<script lang="ts">
	import { getContext } from 'svelte';
	import Gallery from '$lib/components/photos/Gallery.svelte';
	import { locale } from '$lib/locale';
	import { hasPermission } from '$lib/stores/authStore';
	import { TREE_CONTEXT_KEY, type TreeLayoutState } from '../layout.svelte.ts';

	const layoutState = getContext<TreeLayoutState>(TREE_CONTEXT_KEY);
</script>

{#if layoutState.tree}
	{#if $hasPermission('photo:read')}
		{@const tree = layoutState.tree}

		<Gallery id={tree.id} count={tree.files?.length} species={tree.species} />
	{:else}
		<p>{locale.photoReadDenied()}</p>
	{/if}
{/if}
