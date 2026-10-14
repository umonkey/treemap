<script lang="ts">
	import HeatMap from './HeatMap.svelte';
	import { UserHeatMap } from './UserHeatMap.svelte.ts';

	type Props = {
		id: string;
	};

	const { id }: Props = $props();

	const componentState = new UserHeatMap();

	$effect(() => {
		componentState.init(id);
	});
</script>

{#if componentState.loading && componentState.data.length === 0}
	<!-- loading -->
{:else if componentState.data.length > 0}
	<HeatMap data={componentState.data} />
{:else if componentState.error}
	<p>Error loading heat map: {componentState.error}</p>
{:else}
	<p>Error loading heat map.</p>
{/if}
