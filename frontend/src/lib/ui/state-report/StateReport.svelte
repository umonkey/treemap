<script lang="ts">
	import { StateChart } from '$lib/ui';

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
		<StateChart {data} />
	</div>
{/if}

<style>
	.chart {
		height: 200px;
		width: 100%;
	}
</style>
