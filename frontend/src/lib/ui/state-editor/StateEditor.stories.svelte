<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { StateEditor } from '$lib/ui';
	import { DEFAULT_TREE } from '$lib/constants';

	const { Story } = defineMeta({
		title: 'UI/StateEditor',
		component: StateEditor,
		argTypes: {
			tree: { control: 'object' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		tree: DEFAULT_TREE
	}}
/>
