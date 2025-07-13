<script lang="ts">
	import { BarChart } from '$lib/ui';
	import { formatData } from './hooks';

	type Props = {
		title: string;
		data: {
			value: number;
			count: number;
		}[];
	};

	const { data, title }: Props = $props();
</script>

{#if data}
	<h3>{title}</h3>

	<div class="chart">
		<BarChart data={formatData(data)} />
	</div>
{/if}

<style>
	.chart {
		width: 100%;
		aspect-ratio: 2 / 1;
	}

	h3 {
		margin-top: 50px;
	}
</style>
