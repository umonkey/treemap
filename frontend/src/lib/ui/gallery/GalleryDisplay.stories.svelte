<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import GalleryDisplay from './GalleryDisplay.svelte';

	const { Story } = defineMeta({
		title: 'UI/GalleryDisplay',
		component: GalleryDisplay,
		argTypes: {
			files: { control: 'object' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		items: [
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
