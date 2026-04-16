<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import SignOutButton from './SignOutButton.svelte';

	const { Story } = defineMeta({
		title: 'UI/SignOutButton',
		component: SignOutButton
	});
</script>

<Story name="Primary" />
