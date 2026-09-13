<script lang="ts">
	import { getMapContext } from 'svelte-maplibre';
	import { MapMenuBridgeLogic } from './MapMenuBridge.svelte.ts';

	const componentState = new MapMenuBridgeLogic();
	const mapContext = getMapContext();

	$effect(() => {
		const map = mapContext.map;
		if (map) {
			componentState.mount(map);
			return () => {
				componentState.unmount(map);
			};
		}
	});
</script>
