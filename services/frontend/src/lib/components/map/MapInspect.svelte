<script lang="ts">
	import { MapInspectLogic } from './MapInspect.svelte.ts';
	import { mapState } from './MapLibre.svelte.ts';

	const state = new MapInspectLogic();

	$effect(() => {
		const map = mapState.map;
		if (map) {
			state.mount(map);
			return () => {
				state.unmount(map);
			};
		}
	});
</script>
