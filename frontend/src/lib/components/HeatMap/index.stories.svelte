<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import HeatMap from './index.svelte';

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: 'Charts/HeatMap',
		component: HeatMap,
		argTypes: {
			data: { control: 'object' }
		},
		args: {
			data: [
				{
					date: '2023-10-01',
					value: 10
				},
				{
					date: '2023-10-02',
					value: 20
				},
				{
					date: '2023-10-03',
					value: 30
				},
				{
					date: '2023-10-04',
					value: 40
				}
			]
		}
	});
</script>

<!-- More on writing stories with args: https://storybook.js.org/docs/writing-stories/args -->
<Story name="Primary" />
