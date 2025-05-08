<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import YearInput from './YearInput.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/YearInput',
		component: YearInput,
		argTypes: {
			value: { control: 'number' }
		},
		args: {
			onChange: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		value: 2024
	}}
/>
