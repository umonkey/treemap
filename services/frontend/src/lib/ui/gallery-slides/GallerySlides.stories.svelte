<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import GallerySlides from './GallerySlides.svelte';

	const { Story } = defineMeta({
		title: 'UI/GallerySlides',
		component: GallerySlides,
		argTypes: {
			slides: { control: 'object' },
			initialImageId: { control: 'text' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		slides: [
			{
				id: '1',
				small: 'https://placecats.com/neo/500/500',
				large: 'https://placecats.com/neo/500/500',
				label: 'First Image'
			},
			{
				id: '2',
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
				id: '1',
				small: 'https://placecats.com/neo/500/500',
				large: 'https://placecats.com/neo/500/500',
				label: 'First Image'
			},
			{
				id: '2',
				small: 'https://placecats.com/millie/500/500',
				large: 'https://placecats.com/millie/500/500',
				label: 'Second Image'
			}
		]
	}}
/>
