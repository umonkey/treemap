<script lang="ts">
	import { hooks } from './hooks';
	import HeatMap from '../HeatMap/index.svelte';

	const { data, error, loading } = hooks();
</script>

{#if $loading}
	<!-- loading -->
{:else if $data}
	<HeatMap
		title="Recent activity"
		data={$data}
		docs="https://github.com/umonkey/treemap/wiki/Heat-maps"
	/>
{:else if $error}
	<p>Error loading heat map: {error}</p>
{:else}
	<p>Error loading heat map.</p>
{/if}
