<script lang="ts">
	import { MAPS_KEY } from '$lib/env';

	const { lat, lng } = $props<{
		lat: number;
		lng: number;
	}>();
</script>

<iframe
	frameborder="0"
	referrerpolicy="no-referrer-when-downgrade"
	src="https://www.google.com/maps/embed/v1/streetview?key={MAPS_KEY}&location={lat},{lng}"
	allowfullscreen
	title="Street View"
>
</iframe>

<style>
	iframe {
		border: none;
		width: 100%;
		aspect-ratio: 1;
		display: block;
	}
</style>
