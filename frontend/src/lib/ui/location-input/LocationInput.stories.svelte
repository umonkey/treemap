<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import LocationInput from './LocationInput.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/LocationInput',
		component: LocationInput,
		argTypes: {
			value: { control: 'object' },
			hint: { control: 'text' },
			label: { control: 'text' },
			open: { control: 'boolean' }
		},
		args: {
			onChange: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		value: { lat: 40.181389, lng: 44.514444 },
		label: 'Location',
		hint: 'Please choose a location'
	}}
/>

<Story
	name="Open"
	args={{
		value: { lat: 40.181389, lng: 44.514444 },
		label: 'Location',
		hint: 'Please choose a location',
		open: true
	}}
/>
