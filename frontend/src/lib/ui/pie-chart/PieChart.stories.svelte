<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import PieChart from './PieChart.svelte';

	const { Story } = defineMeta({
		title: 'Charts/PieChart',
		component: PieChart,
		argTypes: {
			data: { control: 'object' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		data: [
			{
				label: 'deformed',
				value: 5
			},
			{
				label: 'healthy',
				value: 288
			},
			{
				label: 'sick',
				value: 5
			},
			{
				label: 'stump',
				value: 7
			},
			{
				label: 'unknown',
				value: 2
			}
		]
	}}
/>
