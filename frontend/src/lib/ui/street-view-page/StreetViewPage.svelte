<script lang="ts">
	import Actions from '$lib/components/tree/Actions.svelte';
	import Description from '$lib/components/tree/Description.svelte';
	import Properties from '$lib/components/tree/Properties.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeTabs from '$lib/components/tree/TreeTabs.svelte';
	import StreetView from '$lib/ui/street-view/StreetView.svelte';
	import { loadTree } from './hooks';

	const { id } = $props<{ id: string }>();
	const { loading, data, error, reload } = loadTree();

	$effect(() => {
		reload(id);
	});
</script>

{#if $loading}
	<!-- loading -->
{:else if $error}
	<p>{$error}</p>
{:else if $data}
	<Title id={$data.id} title={$data.species} address={$data.address ?? undefined} padded />
	<TreeTabs tree={$data.id} active="360" />

	<StreetView lat={$data.lat} lng={$data.lon} />

	<Actions tree={$data} />
	<Properties tree={$data} />
	<Description text={$data.notes} />
{/if}
