<script lang="ts">
	import { getStats } from '$lib/api/stats';
	import { statsStore } from '$lib/stores/statsStore';
	import { onMount } from 'svelte';

	onMount(async () => {
		const res = await getStats();
		if (res.data) {
			statsStore.set(res.data);
		}
	});
</script>

<svelte:head>
	<title>Trees of Yerevan</title>
</svelte:head>
