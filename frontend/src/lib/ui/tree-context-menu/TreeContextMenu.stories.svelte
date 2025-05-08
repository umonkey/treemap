<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import TreeContextMenuDisplay from './TreeContextMenuDisplay.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/TreeContextMenu',
		component: TreeContextMenuDisplay,
		argTypes: {
			id: { control: 'text' },
			visible: { control: 'boolean' }
		},
		args: {
			onClose: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		id: 'tree1',
		visible: true
	}}
/>
