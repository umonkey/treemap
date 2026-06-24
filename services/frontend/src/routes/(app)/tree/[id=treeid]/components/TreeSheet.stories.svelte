<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import TreeSheet from './TreeSheet.svelte';

	const { Story } = defineMeta({
		title: 'UI/TreeSheet',
		component: TreeSheet,
		argTypes: {
			tree: { control: 'object' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		tree: {
			id: 'tree1',
			species: 'Fraxinus excelsior',
			state: 'dead',
			address: 'Manukyan str.'
		}
	}}
/>
