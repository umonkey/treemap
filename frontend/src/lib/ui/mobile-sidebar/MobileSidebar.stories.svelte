<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import MobileSidebarDisplay from './MobileSidebarDisplay.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/MobileSidebar',
		component: MobileSidebarDisplay,
		argTypes: {
			visible: { control: 'boolean' },
			last_tree: { control: 'text' },
			search: { control: 'text' }
		},
		args: {
			onClose: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		visible: true,
		last_tree: null
	}}
/>
