<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import LeftSidebar from './LeftSidebar.svelte';

	const { Story } = defineMeta({
		title: 'Layout/LeftSidebar',
		component: LeftSidebar,
		argTypes: {}
	});
</script>

<Story name="Primary" />
