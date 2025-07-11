<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { Chart } from '$lib/ui';

	const { Story } = defineMeta({
		title: 'UI/Chart',
		component: Chart,
		argTypes: {
			type: { control: 'select', options: ['line', 'bar', 'pie'] },
			labels: { control: 'object' },
			data: { control: 'object' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		type: 'bar',
		data: {
			labels: ['January', 'February', 'March', 'April', 'May'],
			datasets: [
				{
					label: '# of Votes',
					data: [12, 19, 3, 5, 2, 3],
					backgroundColor: 'rgba(255, 99, 132, 0.2)',
					borderColor: 'rgba(255, 99, 132, 1)',
					borderWidth: 1
				}
			]
		}
	}}
/>
