<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { LazyImage } from '$lib/ui';

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: 'UI/LazyImage',
		component: LazyImage,
		argTypes: {
			image: { control: 'text' },
			fallback: { control: 'text' },
			alt: { control: 'text' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		src: 'https://placecats.com/200/200',
		fallback: 'https://placehold.co/300x200/EEE/31343C',
		alt: 'example'
	}}
/>

<Story
	name="Default"
	args={{
		src: 'foobar',
		fallback: 'https://placehold.co/300x200/EEE/31343C',
		alt: 'example'
	}}
/>
