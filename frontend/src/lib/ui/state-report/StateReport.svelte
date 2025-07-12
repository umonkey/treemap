<script lang="ts">
	import { PieChart } from '$lib/ui';
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
	<h3>Trees by state</h3>

	<div class="chart">
		<PieChart data={formatData(data)} />
	</div>
{/if}

<style>
	.chart {
		height: 200px;
		width: 100%;
	}

	h3 {
		margin-top: 50px;
	}
</style>
