<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { SignInButton } from '$lib/ui';

	const { Story } = defineMeta({
		title: 'UI/SignInButton',
		component: SignInButton,
	});
</script>

<Story name="Primary" />
