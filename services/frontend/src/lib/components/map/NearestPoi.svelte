<script lang="ts">
	import { stickyPointsLayer } from '$lib/stores/mapLayerStore';
	import { GeoJSON, LineLayer, getMapContext } from 'svelte-maplibre';
	import { NearestPoiLogic } from './NearestPoi.svelte.ts';

	const { distance = 100, zoom = 0 }: { distance?: number; zoom?: number } = $props();

	const componentState = new NearestPoiLogic();
	const mapContext = getMapContext();

	$effect(() => {
		componentState.maxDistance = distance;
	});

	$effect(() => {
		componentState.zoom = zoom;
	});

	$effect(() => {
		const map = mapContext.map;
		if (map) {
			componentState.mount(map);
			return () => {
				componentState.unmount();
			};
		}
	});
</script>

{#if $stickyPointsLayer && componentState.visible && componentState.moving}
	{#each componentState.links as link (link.poi.url)}
		<GeoJSON data={link.line}>
			<LineLayer
				layout={{ 'line-cap': 'round', 'line-join': 'round' }}
				paint={{
					'line-color': '#888888',
					'line-width': 2,
					'line-dasharray': [2, 2]
				}}
			/>
		</GeoJSON>
	{/each}
{/if}
