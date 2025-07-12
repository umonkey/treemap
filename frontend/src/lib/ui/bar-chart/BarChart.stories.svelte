<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import BarChart from './BarChart.svelte';

	const { Story } = defineMeta({
		title: 'Charts/BarChart',
		component: BarChart,
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
				label: '3 m',
				value: 3
			},
			{
				label: '4 m',
				value: 102
			},
			{
				label: '5 m',
				value: 70
			},
			{
				label: '6 m',
				value: 3
			},
			{
				label: '7 m',
				value: 1
			}
		]
	}}
/>
