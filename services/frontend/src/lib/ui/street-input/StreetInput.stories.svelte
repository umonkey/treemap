<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import StreetInput from './StreetInput.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/StreetInput',
		component: StreetInput,
		argTypes: {
			value: { control: 'object' }
		},
		args: {
			onChange: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		value: 'Hello, world.'
	}}
/>
