<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { FileUploader } from '$lib/ui';
	import { fn } from '@storybook/test';

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
