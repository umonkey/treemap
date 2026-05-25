<script lang="ts">
	import { GeoJSON, LineLayer, Marker } from 'svelte-maplibre';
	import { componentState } from './NearestTree.svelte.ts';
</script>

{#if componentState.nearest?.line}
	<GeoJSON data={componentState.nearest.line}>
		<LineLayer
			layout={{ 'line-cap': 'round', 'line-join': 'round' }}
			paint={{
				'line-color': '#888888',
				'line-width': 2,
				'line-dasharray': [2, 2]
			}}
		/>
	</GeoJSON>

	<Marker lngLat={componentState.nearest.midpoint}>
		<div class="distance-label">
			{Math.round(componentState.nearest.distance)}m
		</div>
	</Marker>
{/if}

<style>
	.distance-label {
		background: white;
		color: black;
		padding: 2px 6px;
		border-radius: 4px;
		font-size: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
		pointer-events: none;
		white-space: nowrap;
	}
</style>
