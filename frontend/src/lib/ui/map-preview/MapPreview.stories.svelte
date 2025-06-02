<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { MapPreview } from '$lib/ui';
	import './mocks';

	const { Story } = defineMeta({
		title: 'UI/MapPreview',
		component: MapPreview,
		argTypes: {
			id: { control: 'text' }
		}
	});
</script>

<Story
	name="Mobile"
	args={{
		id: 'gif1'
	}}
/>

<Story
	name="Mobile - Long Text"
	args={{
		id: 'gif2'
	}}
/>
