<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import GalleryPreview from './GalleryPreview.svelte';

	const { Story } = defineMeta({
		title: 'Components/Photos/GalleryPreview',
		component: GalleryPreview,
		argTypes: {
			id: { control: 'text' },
			loading: { control: 'boolean' },
			error: { control: 'text' },
			mapper: { control: 'boolean' },
			files: { control: 'object' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		id: 'tree1',
		loading: false,
		error: '',
		mapper: false,
		files: [
			{ id: '1', small_id: '1', large_id: '1' },
			{ id: '2', small_id: '2', large_id: '2' }
		]
	}}
/>

<Story
	name="Mapper"
	args={{
		id: 'tree1',
		loading: false,
		error: '',
		mapper: true,
		files: [
			{ id: '1', small_id: '1', large_id: '1' },
			{ id: '2', small_id: '2', large_id: '2' }
		]
	}}
/>

<Story
	name="Empty"
	args={{
		id: 'tree1',
		loading: false,
		error: '',
		mapper: false,
		files: []
	}}
/>

<Story
	name="Empty Mapper"
	args={{
		id: 'tree1',
		loading: false,
		error: '',
		mapper: true,
		files: []
	}}
/>

<Story
	name="Loading"
	args={{
		id: 'tree1',
		loading: true,
		error: '',
		mapper: false,
		files: []
	}}
/>
