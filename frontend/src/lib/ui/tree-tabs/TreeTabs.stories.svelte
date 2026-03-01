<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { TreeTabs } from '$lib/ui';

	const { Story } = defineMeta({
		title: 'UI/TreeTabs',
		component: TreeTabs,
		argTypes: {
			tree: { control: 'text' },
			active: { control: 'text' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		tree: 'tree1',
		active: 'details'
	}}
/>

<Story
	name="History"
	args={{
		tree: 'tree1',
		active: 'history'
	}}
/>
