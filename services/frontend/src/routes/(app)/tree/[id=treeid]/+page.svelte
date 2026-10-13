<script lang="ts">
	import { getContext } from 'svelte';
	import { page } from '$app/state';
	import Observations from '$lib/components/observation/Observations.svelte';
	import Actions from './components/Actions.svelte';
	import Properties from './components/Properties.svelte';
	import { TREE_CONTEXT_KEY, type TreeLayoutState } from './layout.svelte.ts';
	import { TreePageState } from './page.svelte.ts';
	import '$lib/styles/variables.css';

	const layoutState = getContext<TreeLayoutState>(TREE_CONTEXT_KEY);
	const pageState = new TreePageState();

	const id = $derived(page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});
</script>

{#if layoutState.tree}
	{@const tree = layoutState.tree}

	<Properties {tree} />

	{#if pageState.observations}
		<Observations observation={pageState.observations} />
	{/if}

	<hr />

	<Actions {tree} />
{/if}

<style>
	hr {
		margin: 0;
		padding: 0;
		border: none;
		border-bottom: 1px solid rgba(128, 128, 128, 0.2);
	}
</style>
