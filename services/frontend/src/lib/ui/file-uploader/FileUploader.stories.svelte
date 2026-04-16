<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { fn } from '@storybook/test';
	import FileUploader from './FileUploader.svelte';

	const { Story } = defineMeta({
		title: 'UI/FileUploader',
		component: FileUploader,
		argTypes: {
			disabled: { control: 'boolean' },
			small: { control: 'boolean' }
		},
		args: {
			onChange: fn(),
			onBusy: fn()
		}
	});
</script>

<Story name="Primary" />

<Story
	name="Small"
	args={{
		small: true
	}}
/>

<Story
	name="Label"
	args={{
		label: 'Update profile picture:',
		single: true
	}}
/>
