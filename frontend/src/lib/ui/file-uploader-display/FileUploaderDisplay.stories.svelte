<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { FileUploaderDisplay } from '$lib/ui';
	import { fn } from '@storybook/test';
	import IMAGE1 from '$lib/assets/explorer-mode-500px.jpeg';
	import IMAGE2 from '$lib/assets/learning-mode-500px.jpeg';
	import IMAGE3 from '$lib/assets/reports-mode-500px.jpeg';

	const { Story } = defineMeta({
		title: 'UI/FileUploaderDisplay',
		component: FileUploaderDisplay,
		argTypes: {
			items: { control: 'object' },
			small: { control: 'boolean' }
		},
		args: {
			onRetry: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		items: [
			{
				src: IMAGE1,
				busy: false,
				error: false
			},
			{
				src: IMAGE2,
				busy: true,
				error: false
			},
			{
				src: IMAGE3,
				busy: false,
				error: true
			}
		]
	}}
/>

<Story
	name="Scroll"
	args={{
		items: [
			{
				src: IMAGE1,
				busy: false,
				error: false
			},
			{
				src: IMAGE2,
				busy: true,
				error: false
			},
			{
				src: IMAGE3,
				busy: false,
				error: true
			},
			{
				src: IMAGE1,
				busy: false,
				error: false
			},
			{
				src: IMAGE2,
				busy: true,
				error: false
			},
			{
				src: IMAGE3,
				busy: false,
				error: true
			},
			{
				src: IMAGE1,
				busy: false,
				error: false
			},
			{
				src: IMAGE2,
				busy: true,
				error: false
			},
			{
				src: IMAGE3,
				busy: false,
				error: true
			},
			{
				src: IMAGE1,
				busy: false,
				error: false
			},
			{
				src: IMAGE2,
				busy: true,
				error: false
			},
			{
				src: IMAGE3,
				busy: false,
				error: true
			},
			{
				src: IMAGE1,
				busy: false,
				error: false
			},
			{
				src: IMAGE2,
				busy: true,
				error: false
			},
			{
				src: IMAGE3,
				busy: false,
				error: true
			},
			{
				src: IMAGE1,
				busy: false,
				error: false
			},
			{
				src: IMAGE2,
				busy: true,
				error: false
			},
			{
				src: IMAGE3,
				busy: false,
				error: true
			}
		]
	}}
/>

<Story
	name="Small"
	args={{
		items: [
			{
				src: IMAGE1,
				busy: false,
				error: false
			},
			{
				src: IMAGE2,
				busy: true,
				error: false
			},
			{
				src: IMAGE3,
				busy: false,
				error: true
			}
		],
		small: true
	}}
/>
