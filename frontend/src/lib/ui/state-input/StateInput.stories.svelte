<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import StateInput from './StateInput.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/StateInput',
		component: StateInput,
		argTypes: {
			value: { control: 'object' },
			label: { control: 'text' }
		},
		args: {
			onChange: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		value: 'unknown',
		label: 'State'
	}}
/>
