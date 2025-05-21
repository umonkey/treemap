<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { FileUploader } from '$lib/ui';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/FileUploader',
		component: FileUploader,
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
