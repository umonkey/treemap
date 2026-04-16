<script lang="ts">
	import Chart from '../chart/Chart.svelte';
	import { formatChartProps } from './hooks';

	type Props = {
		data: {
			label: string;
			value: number;
		}[];
	};

	const { data }: Props = $props();
</script>

<Chart {...formatChartProps(data)} />
