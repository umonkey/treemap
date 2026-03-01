<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import CheckInput from './CheckInput.svelte';

	const { Story } = defineMeta({
		title: 'UI/CheckInput',
		component: CheckInput,
		argTypes: {
			value: { control: 'boolean' },
			label: { control: 'text' },
			hint: { control: 'text' },
			help: { control: 'text' },
			disabled: { control: 'boolean' }
		}
	});
</script>

<Story
	name="Off"
	args={{
		value: false,
		label: 'Toggle switch'
	}}
/>

<Story
	name="On"
	args={{
		value: true,
		label: 'Toggle switch'
	}}
/>

<Story
	name="With Hint"
	args={{
		value: false,
		label: 'Toggle switch',
		hint: 'This is a hint'
	}}
/>

<Story
	name="Disabled"
	args={{
		value: true,
		label: 'Disabled switch',
		disabled: true
	}}
/>
