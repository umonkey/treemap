<script lang="ts">
	import { hooks } from './hooks';
	import HeatMap from '../HeatMap/index.svelte';

	const { data, error, loading } = hooks();
</script>

{#if $loading}
	<!-- loading -->
{:else if $data}
	<HeatMap data={$data} />
{:else if $error}
	<p>Error loading heat map: {error}</p>
{:else}
	<p>Error loading heat map.</p>
{/if}
