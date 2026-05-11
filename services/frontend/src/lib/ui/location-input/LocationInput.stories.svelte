<script lang="ts" module>
	/* eslint-disable @typescript-eslint/no-explicit-any */
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import LocationInput from './LocationInput.svelte';

	const { Story } = defineMeta({
		title: 'UI/LocationInput',
		component: LocationInput,
		argTypes: {
			value: { control: 'object' },
			hint: { control: 'text' },
			label: { control: 'text' }
		},
		args: {
			label: 'Location',
			hint: 'Please choose a location',
			value: { lat: 40.181389, lng: 44.514444 }
		}
	});
</script>

{#snippet padded(args: any)}
	<div class="padded">
		<LocationInput {...args} />
	</div>
{/snippet}

<Story name="Primary" args={{}} template={padded} />

<style>
	.padded {
		padding: 10px;
		max-width: 480px;
		box-sizing: border-box;
	}
</style>
