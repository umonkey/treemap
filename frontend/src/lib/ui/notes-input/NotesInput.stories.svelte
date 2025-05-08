<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import NotesInput from './NotesInput.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/NotesInput',
		component: NotesInput,
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
