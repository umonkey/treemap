<script lang="ts">
	import StreetView from '$lib/ui/street-view/StreetView.svelte';
	import Actions from '../components/Actions.svelte';
	import Description from '../components/Description.svelte';
	import Properties from '../components/Properties.svelte';
	import Title from '../components/Title.svelte';
	import TreeTabs from '../components/TreeTabs.svelte';
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
