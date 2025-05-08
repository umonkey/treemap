<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import FilteredChangeList from './FilteredChangeList.svelte';

	const { Story } = defineMeta({
		title: 'UI/FilteredChangeList',
		component: FilteredChangeList,
		argTypes: {
			changes: { control: 'object' },
			name: { control: 'text' }
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
			},
			{
				id: 'change3',
				tree_id: 'tree1',
				name: 'state',
				value: 'dead',
				added_at: 1744971200,
				added_by: 'Charlie'
			}
		],
		name: 'state'
	}}
/>
