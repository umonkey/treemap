<script lang="ts" module>
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
			label: 'Location',
			hint: 'Please choose a location',
			value: { lat: 40.181389, lng: 44.514444 },
			onChange: fn()
		}
	});
</script>

{#snippet padded(args)}
	<div class="padded">
		<LocationInput {...args} />
	</div>
{/snippet}

<Story name="Primary" args={{ simpleChild: true }} template={padded} />

<Story
	name="Open"
	args={{
		open: true,
		simpleChild: true
	}}
	template={padded}
/>

<style>
	.padded {
		padding: 10px;
		max-width: 480px;
		box-sizing: border-box;
	}
</style>
