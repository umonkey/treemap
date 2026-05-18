<script lang="ts">
	import MapPreview from './MapPreview.svelte';
	import { page } from '$app/stores';

	const id = $derived($page.params.id as string);
</script>

<svelte:head>
	<title>Report {id} - Trees of Yerevan</title>
</svelte:head>

<MapPreview {id} />
