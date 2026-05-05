<script lang="ts">
	import { onMount } from 'svelte';
	import { mapState } from './MapLibre.svelte.ts';

	let rotation = $state(0);

	function zoomIn() {
		mapState.map?.zoomIn();
	}

	function zoomOut() {
		mapState.map?.zoomOut();
	}

	function resetNorth() {
		mapState.map?.resetNorthPitch();
	}

	onMount(() => {
		const updateRotation = () => {
			if (mapState.map) {
				rotation = mapState.map.getBearing();
			}
		};

		mapState.map?.on('rotate', updateRotation);
		return () => {
			mapState.map?.off('rotate', updateRotation);
		};
	});
</script>

<div class="navigation-wrapper">
	<div class="maplibregl-ctrl maplibregl-ctrl-group">
		<button
			class="maplibregl-ctrl-zoom-in"
			type="button"
			title="Zoom in"
			onclick={zoomIn}
			aria-label="Zoom in"
		>
			<span class="maplibregl-ctrl-icon" aria-hidden="true"></span>
		</button>
		<button
			class="maplibregl-ctrl-zoom-out"
			type="button"
			title="Zoom out"
			onclick={zoomOut}
			aria-label="Zoom out"
		>
			<span class="maplibregl-ctrl-icon" aria-hidden="true"></span>
		</button>
		<button
			class="maplibregl-ctrl-compass"
			type="button"
			title="Reset North"
			onclick={resetNorth}
			aria-label="Reset North"
		>
			<span
				class="maplibregl-ctrl-icon"
				aria-hidden="true"
				style:transform="rotate({rotation * -1}deg)"
			></span>
		</button>
	</div>
</div>

<style>
	.navigation-wrapper {
		position: absolute;
		right: 10px;
		top: 50%;
		transform: translateY(-50%);
		z-index: 10;
		margin-right: env(safe-area-inset-right);
	}
</style>
