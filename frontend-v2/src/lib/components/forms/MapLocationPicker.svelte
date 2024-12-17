<script lang="ts">
	import Map from '$lib/components/Map.svelte';

	const { center } = $props();
</script>

<div class="map">
	<Map {center} />
</div>

<style>
	.map {
		background-color: var(--form-background);
		width: 100%;
		aspect-ratio: 4/3;
		margin: var(--gap) 0;
	}
</style>
