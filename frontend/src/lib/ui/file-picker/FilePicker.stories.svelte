<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import FilePicker from './FilePicker.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/FilePicker',
		component: FilePicker,
		argTypes: {
			disabled: { control: 'boolean' }
		},
		args: {
			onFileSelected: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		value: 'no comment'
	}}
/>
