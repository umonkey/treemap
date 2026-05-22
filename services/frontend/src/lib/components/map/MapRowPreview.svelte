<script lang="ts">
	import { GeoJSON, CircleLayer, LineLayer } from 'svelte-maplibre';
	import { mapRowState } from '$lib/stores/mapRowState.svelte';
</script>

{#if mapRowState.previewData}
	<GeoJSON data={mapRowState.previewData}>
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
{/if}
