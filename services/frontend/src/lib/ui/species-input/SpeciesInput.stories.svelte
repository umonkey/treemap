<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import SpeciesInput from './SpeciesInput.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/SpeciesInput',
		component: SpeciesInput,
		argTypes: {
			value: { control: 'object' }
		},
		args: {
			onChange: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		value: 'Hello, world.'
	}}
/>
