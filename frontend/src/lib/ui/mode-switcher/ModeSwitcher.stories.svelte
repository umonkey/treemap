<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ModeSwitcher from './ModeSwitcher.svelte';

	const { Story } = defineMeta({
		title: 'UI/ModeSwitcher',
		component: ModeSwitcher
	});
</script>

<Story name="Primary" />
