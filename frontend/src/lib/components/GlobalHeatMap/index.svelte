<script lang="ts">
	import { locale } from '$lib/locale';
	import { onMount } from 'svelte';
	import HeatMap from '../HeatMap/index.svelte';
	import { globalHeatMapState } from './index.svelte.ts';

	onMount(() => {
		globalHeatMapState.init();
	});
</script>

{#if globalHeatMapState.loading && globalHeatMapState.data.length === 0}
	<!-- loading -->
{:else if globalHeatMapState.data.length > 0}
	<HeatMap
		title={locale.globalHeatmapHeader()}
		data={globalHeatMapState.data}
		docs="https://github.com/umonkey/treemap/wiki/Heat-maps"
	/>
{:else if globalHeatMapState.error}
	<p>Error loading heat map: {globalHeatMapState.error}</p>
{:else}
	<p>Error loading heat map.</p>
{/if}
