<script lang="ts">
	import { getMapContext } from 'svelte-maplibre';
	import { MapEventBridgeLogic } from './MapEventBridge.svelte.ts';

	const componentState = new MapEventBridgeLogic();
	const mapContext = getMapContext();

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
