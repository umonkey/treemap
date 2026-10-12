<script lang="ts">
	import { getContext } from 'svelte';
	import StreetView from '$lib/ui/street-view/StreetView.svelte';
	import Actions from '../components/Actions.svelte';
	import Description from '../components/Description.svelte';
	import Properties from '../components/Properties.svelte';
	import { TREE_CONTEXT_KEY, type TreeLayoutState } from '../layout.svelte.ts';

	const layoutState = getContext<TreeLayoutState>(TREE_CONTEXT_KEY);
	const tree = $derived(layoutState.tree);
</script>

{#if tree}
	<StreetView lat={tree.lat} lng={tree.lon} />

	<Actions {tree} />
	<Properties {tree} />
	<Description text={tree.notes} />
{/if}
