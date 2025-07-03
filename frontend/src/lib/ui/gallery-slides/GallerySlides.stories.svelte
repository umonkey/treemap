<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { GallerySlides } from '$lib/ui';

	const { Story } = defineMeta({
		title: 'UI/GallerySlides',
		component: GallerySlides,
		argTypes: {
			slides: { control: 'object' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		slides: [
			{
				small: 'https://placecats.com/neo/500/500',
				large: 'https://placecats.com/neo/500/500',
				label: 'First Image'
			},
			{
				small: 'https://placecats.com/millie/500/500',
				large: 'https://placecats.com/millie/500/500',
				label: 'Second Image'
			}
		]
	}}
	parameters={{
		viewport: {
			defaultViewport: 'fullscreen'
		}
	}}
/>

<Story
	name="Mobile"
	args={{
		slides: [
			{
				small: 'https://placecats.com/neo/500/500',
				large: 'https://placecats.com/neo/500/500',
				label: 'First Image'
			},
			{
				small: 'https://placecats.com/millie/500/500',
				large: 'https://placecats.com/millie/500/500',
				label: 'Second Image'
			}
		]
	}}
/>
