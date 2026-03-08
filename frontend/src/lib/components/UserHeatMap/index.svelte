<script lang="ts">
	import { locale } from '$lib/locale';
	import HeatMap from '../HeatMap/index.svelte';
	import { hooks } from './hooks';

	type Props = {
		id: string;
	};

	const { id }: Props = $props();
	const { data, error, loading } = hooks(id);
</script>

{#if $loading}
	<!-- loading -->
{:else if $data}
	<HeatMap
		title={locale.globalHeatmapHeader()}
		data={$data}
		docs="https://github.com/umonkey/treemap/wiki/Heat-maps"
	/>
{:else if $error}
	<p>Error loading heat map: {error}</p>
{:else}
	<p>Error loading heat map.</p>
{/if}
