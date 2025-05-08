<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ChangeList from './ChangeList.svelte';

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: 'UI/ChangeList',
		component: ChangeList,
		argTypes: {
			changes: { control: 'object' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		changes: [
			{
				id: 'change1',
				tree_id: 'tree1',
				name: 'species',
				value: 'Fraxinus',
				added_at: 1746637613,
				added_by: 'Alice'
			},
			{
				id: 'change2',
				tree_id: 'tree1',
				name: 'state',
				value: 'dead',
				added_at: 1745971200,
				added_by: 'Bob'
			}
		]
	}}
/>
