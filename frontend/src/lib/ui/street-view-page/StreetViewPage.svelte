<script lang="ts">
	import { StreetView } from '$lib/ui';
	import Actions from '$lib/components/tree/Actions.svelte';
	import Description from '$lib/components/tree/Description.svelte';
	import Links from '$lib/components/tree/Links.svelte';
	import Properties from '$lib/components/tree/Properties.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import TreeTabs from '$lib/components/tree/TreeTabs.svelte';
	import { loadTree } from '$lib/hooks/loadTree';

	const { id } = $props<{ id: string }>();
	const { loading, data, error, reload } = loadTree();

	$effect(() => reload(id));
</script>

{#if $loading}
	<!-- loading -->
{:else if $error}
	<p>{$error}</p>
{:else}
	<Title title={$data.species} address={$data.address} padded />
	<TreeTabs tree={$data.id} active="360" />

	<StreetView lat={$data.lat} lng={$data.lon} />

	<Actions tree={$data} />
	<Properties tree={$data} />
	<Links tree={$data} />
	<Description text={$data.notes} />
	<TreeContextMenu id={$data.id} />
{/if}
