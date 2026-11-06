<script lang="ts">
	import { genusLayer } from '$lib/stores/mapLayerStore';
	import type { SymbolLayerSpecification } from 'maplibre-gl';
	import { SymbolLayer } from 'svelte-maplibre';

	const layout: SymbolLayerSpecification['layout'] = {
		'text-field': ['get', 'genus'],
		'text-font': ['Noto Sans Regular'],
		'text-size': ['interpolate', ['linear'], ['zoom'], 18, 11, 22, 16],
		'text-anchor': 'left',
		'text-offset': [1.2, 0]
	};
</script>

{#if $genusLayer}
	<SymbolLayer
		id="tree-genus"
		minzoom={18}
		filter={['has', 'genus']}
		{layout}
		paint={{
			'text-color': '#1f2937',
			'text-halo-color': '#ffffff',
			'text-halo-width': 1.5
		}}
	/>
{/if}
