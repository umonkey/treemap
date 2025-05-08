<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Buttons from './Buttons.svelte';
	import { Button } from '$lib/ui';

	const { Story } = defineMeta({
		title: 'UI/Buttons',
		component: Buttons,
		argTypes: {},
		args: {}
	});
</script>

<Story name="Primary">
	<Button label="Click Me" type="submit" />
	<Button label="Cancel" type="cancel" />
</Story>
