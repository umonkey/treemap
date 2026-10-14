<script lang="ts">
	import { GeoJSON, LineLayer } from 'svelte-maplibre';
	import { TrackLayerState } from './TrackLayer.svelte.ts';

	const { panoramaId }: { panoramaId: string } = $props();
	const componentState = new TrackLayerState();

	$effect(() => {
		componentState.reload(panoramaId);
	});
</script>

{#if componentState.trackGeoJson}
	<GeoJSON data={componentState.trackGeoJson}>
		<!--
			MapBox/MapLibre paint order is the order layers were added, not the order of this
			markup. Both layers load asynchronously, so whichever resolves last would normally
			paint on top. `beforeLayerType` is resolved once, when this layer is added: if the
			sequence line already exists the track is inserted beneath it, otherwise the track is
			appended and the later-added sequence line lands on top. Either way the track stays
			below the blue image layer.
		-->
		<LineLayer
			id="panorama-track"
			beforeLayerType={(layer) => layer.id === 'panorama-sequence'}
			paint={{
				'line-color': '#9ca3af',
				'line-width': 6,
				'line-opacity': 0.5
			}}
		/>
	</GeoJSON>
{/if}
