<script lang="ts">
	import type { ILatLng } from '$lib/types';
	import { GeoJSON, CircleLayer, LineLayer, getMapContext } from 'svelte-maplibre';
	import MapLibre from '$lib/components/map/MapLibre.svelte';
	import { previewState } from './MapRowPreview.svelte.ts';

	const { start, end, count } = $props<{
		start: ILatLng;
		end: ILatLng;
		count: number;
	}>();

	$effect(() => previewState.update(start, end, count));
</script>

<div class="container">
	<MapLibre>
		<GeoJSON data={previewState.data}>
			<LineLayer
				layout={{ 'line-cap': 'round', 'line-join': 'round' }}
				paint={{
					'line-color': '#007cbf',
					'line-width': 2,
					'line-opacity': 0.8
				}}
			/>

			<CircleLayer
				paint={{
					'circle-color': '#228B22',
					'circle-radius': [
						'interpolate',
						['exponential', 2],
						['zoom'],
						10,
						['*', 5, 0.00428],
						22,
						['*', 5, 17.534]
					],
					'circle-opacity': 0.5,
					'circle-pitch-alignment': 'map',
					'circle-pitch-scale': 'map'
				}}
			/>
		</GeoJSON>
	</MapLibre>
</div>

<style>
	.container {
		background-color: var(--form-background);
		width: 100%;
		aspect-ratio: 4/3;
		margin: var(--gap) 0;
		position: relative;
	}
</style>
