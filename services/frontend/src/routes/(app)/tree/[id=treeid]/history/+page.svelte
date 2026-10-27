<script lang="ts">
	import { getContext } from 'svelte';
	import { page } from '$app/state';
	import { TREE_CONTEXT_KEY, type TreeLayoutState } from '../layout.svelte.ts';
	import ChangeList from './ChangeList.svelte';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	const layoutState = getContext<TreeLayoutState>(TREE_CONTEXT_KEY);
	const tree = $derived(layoutState.tree);

	const id = $derived(page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});
</script>

{#if pageState.loading && !tree}
	<p>Loading...</p>
{:else if pageState.error}
	<p>Error loading tree: {pageState.error.description}</p>
{:else if tree}
	<ChangeList changes={pageState.changes} />
{/if}
