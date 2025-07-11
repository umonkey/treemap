<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import StateChart from './StateChart.svelte';

	const { Story } = defineMeta({
		title: 'Charts/StateChart',
		component: StateChart,
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
				state: 'deformed',
				count: 5
			},
			{
				state: 'healthy',
				count: 288
			},
			{
				state: 'sick',
				count: 5
			},
			{
				state: 'stump',
				count: 7
			},
			{
				state: 'unknown',
				count: 2
			}
		]
	}}
/>
