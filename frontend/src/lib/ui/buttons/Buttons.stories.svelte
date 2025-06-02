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
	<Button type="submit">Click Me</Button>
	<Button type="cancel">Cancel</Button>
</Story>
