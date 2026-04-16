<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import TextInput from './TextInput.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/TextInput',
		component: TextInput,
		argTypes: {
			value: { control: 'text' },
			placeholder: { control: 'text' },
			label: { control: 'text' },
			type: { control: 'select', options: ['text', 'email', 'password'] },
			multiline: { control: 'boolean' },
			hint: { control: 'text' },
			help: { control: 'text' }
		},
		args: {
			onChange: fn()
		}
	});
</script>

<!-- More on writing stories with args: https://storybook.js.org/docs/writing-stories/args -->
<Story
	name="SingleLine"
	args={{
		label: 'Some text',
		value: 'not set',
		hint: 'Please enter a line of text.',
		help: 'https://www.google.com/'
	}}
/>

<!-- More on writing stories with args: https://storybook.js.org/docs/writing-stories/args -->
<Story
	name="MultiLine"
	args={{
		label: 'Some text',
		value: 'not set',
		hint: 'Please enter multiple lines of text.',
		multiline: true,
		help: 'https://www.google.com/'
	}}
/>
