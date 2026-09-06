<script lang="ts">
	import { CircleLayer, GeoJSON, LineLayer } from 'svelte-maplibre';
	import type { FeatureCollection } from 'geojson';
	import { PanoramaAlignmentLayerLogic } from './PanoramaAlignmentLayer.svelte.ts';

	const {
		geoJson,
		hintsGeoJson,
		latOffset = 0,
		lonOffset = 0,
		baseLatOffset = 0,
		baseLonOffset = 0
	}: {
		geoJson?: FeatureCollection;
		hintsGeoJson?: FeatureCollection;
		latOffset?: number;
		lonOffset?: number;
		baseLatOffset?: number;
		baseLonOffset?: number;
	} = $props();

	const componentState = new PanoramaAlignmentLayerLogic();

	$effect(() => {
		componentState.geoJson = geoJson;
		componentState.hintsGeoJson = hintsGeoJson;
		componentState.latOffset = latOffset;
		componentState.lonOffset = lonOffset;
		componentState.baseLatOffset = baseLatOffset;
		componentState.baseLonOffset = baseLonOffset;
	});
</script>

{#if componentState.shiftedGeoJson}
	<GeoJSON data={componentState.shiftedGeoJson}>
		<LineLayer
			id="panorama-alignment-sequence"
			filter={['==', ['get', 'kind'], 'sequence']}
			paint={{
				'line-color': '#007aff',
				'line-width': 4,
				'line-opacity': 0.7
			}}
		/>
		<LineLayer
			id="panorama-alignment-hints"
			filter={['==', ['get', 'kind'], 'hint']}
			paint={{
				'line-color': '#22c55e',
				'line-width': 2,
				'line-opacity': 0.8
			}}
		/>
		<CircleLayer
			id="panorama-alignment-images"
			filter={['==', ['get', 'kind'], 'image']}
			paint={{
				'circle-color': '#007aff',
				'circle-radius': 6,
				'circle-stroke-width': 1,
				'circle-stroke-color': '#ffffff',
				'circle-opacity': 0.9
			}}
		/>
	</GeoJSON>
{/if}
