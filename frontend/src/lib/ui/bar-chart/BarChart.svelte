<script lang="ts">
	import { Chart } from '$lib/ui';
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
