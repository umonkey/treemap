<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { StreetView } from '$lib/ui';

	const { Story } = defineMeta({
		title: 'UI/StreetView',
		component: StreetView,
		argTypes: {
			lat: { control: 'number' },
			lng: { control: 'number' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		lat: 40.2041892,
		lng: 44.5177415
	}}
/>
