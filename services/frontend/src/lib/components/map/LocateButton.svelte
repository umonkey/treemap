<script lang="ts">
	import { mapBus } from '$lib/buses/mapBus';
	import Icon from '$lib/icons/LocationIcon.svelte';
	import { locationStore } from '$lib/stores/locationStore';
	import { Control } from 'svelte-maplibre';

	function handleLocate() {
		if ($locationStore) {
			mapBus.emit('move', {
				lat: $locationStore.lat,
				lng: $locationStore.lng
			});
		}
	}
</script>

<Control position="top-left">
	<div class="maplibregl-ctrl-group">
		<button
			type="button"
			onclick={handleLocate}
			title="Locate me"
			disabled={!$locationStore}
			class:inactive={!$locationStore}
		>
			<Icon />
		</button>
	</div>
</Control>

<style>
	button {
		padding: 4px;
		color: #000;
	}

	.inactive {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
