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
	<h3>Objects by state</h3>

	<p>This chart includes existing trees and places where trees were once growing but were cut down.  This shows the potential for additional panting.</p>

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
