<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import GalleryPreviewDisplay from './GalleryPreviewDisplay.svelte';

	const { Story } = defineMeta({
		title: 'Components/Photos/GalleryPreview',
		component: GalleryPreviewDisplay,
		argTypes: {
			loading: { control: 'boolean' },
			error: { control: 'text' },
			mapper: { control: 'boolean' },
			tree_id: { control: 'text' },
			files: { control: 'object' }
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
		files: [
			{ id: '1', small_id: '1', large_id: '1' },
			{ id: '2', small_id: '2', large_id: '2' }
		]
	}}
/>

<Story
	name="Mapper"
	args={{
		loading: false,
		error: '',
		mapper: true,
		tree_id: 'tree1',
		files: [
			{ id: '1', small_id: '1', large_id: '1' },
			{ id: '2', small_id: '2', large_id: '2' }
		]
	}}
/>

<Story
	name="Empty"
	args={{
		loading: false,
		error: '',
		mapper: false,
		tree_id: 'tree1',
		files: []
	}}
/>

<Story
	name="Empty Mapper"
	args={{
		loading: false,
		error: '',
		mapper: true,
		tree_id: 'tree1',
		files: []
	}}
/>

<Story
	name="Loading"
	args={{
		loading: true,
		error: '',
		mapper: false,
		tree_id: 'tree1',
		files: []
	}}
/>
