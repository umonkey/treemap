<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { TreeTabs } from '$lib/ui';

	const { Story } = defineMeta({
		title: 'UI/TreeTabs',
		component: TreeTabs,
		argTypes: {
			tree: { control: 'numer' },
			active: { control: 'text' },
			comment_count: { control: 'number' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		tree: 'tree1',
		active: 'details',
		comment_count: 76
	}}
/>

<Story
	name="Comments"
	args={{
		tree: 'tree1',
		active: 'comments',
		comment_count: 76
	}}
/>

<Story
	name="No Comments"
	args={{
		tree: 'tree1',
		active: 'comments'
	}}
/>
