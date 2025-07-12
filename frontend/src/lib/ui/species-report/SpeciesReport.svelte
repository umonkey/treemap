<script lang="ts">
	import { BarChart } from '$lib/ui';
	import { formatData } from './hooks';

	type Props = {
		data: {
			state: string;
			count: number;
		}[];
	};

	const { data }: Props = $props();
</script>

{#if data}
	<h3>Trees by species</h3>

	<div class="chart">
		<BarChart data={formatData(data)} />
	</div>
{/if}

<style>
	.chart {
		width: 100%;
		aspect-ratio: 1/1;
	}
</style>
