<script lang="ts">
	import Header from '$lib/components/tree/Header.svelte';

	const { data } = $props();
	const { lat, lng } = data;
</script>

<Header title="Add Tree" />

<div class="container">
	<p>Here will be the form to add a new tree.</p>
	<p>Lat={lat}, Lng={lng}</p>
</div>

<style>
	.container {
		padding: 0 var(--gap);
	}
</style>
