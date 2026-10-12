<script lang="ts">
	import { getContext } from 'svelte';
	import { page } from '$app/state';
	import Observations from '$lib/components/observation/Observations.svelte';
	import Description from './components/Description.svelte';
	import { pageState } from './hooks.svelte.ts';
	import { TREE_CONTEXT_KEY, type TreeLayoutState } from './layout.svelte.ts';

	const layoutState = getContext<TreeLayoutState>(TREE_CONTEXT_KEY);
	const tree = $derived(layoutState.tree);

	const id = $derived(page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});
</script>

{#if tree}
	<Observations observation={pageState.observation || null} />
	<Description text={tree.notes} />
{/if}
