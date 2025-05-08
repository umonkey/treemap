<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import GalleryPreviewDisplay from './GalleryPreviewDisplay.svelte';

	const { Story } = defineMeta({
		title: 'UI/GalleryPreview',
		component: GalleryPreviewDisplay,
		argTypes: {
			loading: { control: 'boolean' },
			error: { control: 'text' },
			mapper: { control: 'boolean' },
			tree_id: { control: 'text' },
			images: { control: 'object' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		loading: false,
		error: '',
		mapper: false,
		tree_id: 'tree1',
		images: ['https://placecats.com/neo/500/500', 'https://placecats.com/millie/500/500']
	}}
/>

<Story
	name="Mapper"
	args={{
		loading: false,
		error: '',
		mapper: true,
		tree_id: 'tree1',
		images: ['https://placecats.com/neo/500/500', 'https://placecats.com/millie/500/500']
	}}
/>

<Story
	name="Empty"
	args={{
		loading: false,
		error: '',
		mapper: false,
		tree_id: 'tree1',
		images: []
	}}
/>

<Story
	name="Empty Mapper"
	args={{
		loading: false,
		error: '',
		mapper: true,
		tree_id: 'tree1',
		images: []
	}}
/>
