<script lang="ts">
	import { getMapContext } from 'svelte-maplibre';
	import { MapEventBridge } from './MapEventBridge.svelte.ts';

	const componentState = new MapEventBridge();
	const mapContext = getMapContext();

	$effect(() => {
		const map = mapContext.map;
		if (map) {
			return componentState.init(map);
		}
	});
</script>
