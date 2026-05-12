<script lang="ts">
	import { GeoJSON, LineLayer } from 'svelte-maplibre';
	import { componentState } from './MoveLine.svelte.ts';
</script>

{#if componentState.line}
	<GeoJSON data={componentState.line}>
		<LineLayer
			layout={{ 'line-cap': 'round', 'line-join': 'round' }}
			paint={{
				'line-color': '#007cbf',
				'line-width': 2,
				'line-dasharray': [2, 2]
			}}
		/>
	</GeoJSON>
{/if}
