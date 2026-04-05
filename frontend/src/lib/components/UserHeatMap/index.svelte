<script lang="ts">
	import { locale } from '$lib/locale';
	import HeatMap from '../HeatMap/index.svelte';
	import { userHeatMapState } from './index.svelte.ts';

	type Props = {
		id: string;
	};

	const { id }: Props = $props();

	$effect(() => {
		userHeatMapState.init(id);
	});
</script>

{#if userHeatMapState.loading && userHeatMapState.data.length === 0}
	<!-- loading -->
{:else if userHeatMapState.data.length > 0}
	<HeatMap
		title={locale.globalHeatmapHeader()}
		data={userHeatMapState.data}
		docs="https://github.com/umonkey/treemap/wiki/Heat-maps"
	/>
{:else if userHeatMapState.error}
	<p>Error loading heat map: {userHeatMapState.error}</p>
{:else}
	<p>Error loading heat map.</p>
{/if}
