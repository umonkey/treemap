<script lang="ts">
	import { GlobalHeatMap } from './GlobalHeatMap.svelte.ts';
	import HeatMap from './HeatMap.svelte';

	const componentState = new GlobalHeatMap();

	$effect(() => {
		componentState.init();
	});
</script>

{#if componentState.loading && componentState.data.length === 0}
	<!-- loading -->
{:else if componentState.data.length > 0}
	<HeatMap data={componentState.data} />
{:else if componentState.error}
	<p>Error loading heat map: {componentState.error}</p>
{:else}
	<p>Error loading heat map.</p>
{/if}
