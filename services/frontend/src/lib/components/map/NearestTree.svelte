<script lang="ts">
	import { GeoJSON, LineLayer } from 'svelte-maplibre';
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
{/if}
