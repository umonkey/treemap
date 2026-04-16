<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import NumberInput from './NumberInput.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/NumberInput',
		component: NumberInput,
		argTypes: {
			value: { control: 'number' },
			placeholder: { control: 'text' },
			label: { control: 'text' },
			type: { control: 'select', options: ['text', 'email', 'password'] },
			autofocus: { control: 'boolean' },
			hint: { control: 'text' },
			help: { control: 'text' }
		},
		args: {
			onChange: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		label: 'Some text',
		value: 1.23,
		hint: 'Please enter a number.',
		help: 'https://www.google.com/'
	}}
/>
