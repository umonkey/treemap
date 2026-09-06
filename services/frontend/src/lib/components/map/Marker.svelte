<script lang="ts">
	import { onMount } from 'svelte';
	import ICON from '$lib/assets/marker-icon-2x.png';
	import { Marker } from 'svelte-maplibre';
	import { MarkerLogic } from './Marker.svelte.ts';

	let componentState = $state<MarkerLogic>();

	onMount(() => {
		componentState = new MarkerLogic();

		return () => {
			componentState?.destroy();
			componentState = undefined;
		};
	});
</script>

{#if componentState?.pin}
	<Marker lngLat={componentState.pin} offset={[0, -16]}>
		<img src={ICON} alt="You are here" />
	</Marker>
{/if}

<style>
	img {
		width: 25px;
		height: 41px;
	}
</style>
