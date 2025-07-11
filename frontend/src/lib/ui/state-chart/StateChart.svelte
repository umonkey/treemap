<script lang="ts">
	import { Chart } from '$lib/ui';
	import { formatChartProps } from './hooks';

	type Props = {
		data: {
			state: string;
			count: number;
		}[];
	};

	const { data }: Props = $props();
</script>

<Chart {...formatChartProps(data)} />
