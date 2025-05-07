<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import AddressInput from './AddressInput.svelte';
	import { fn } from '@storybook/test';

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: 'UI/AddressInput',
		component: AddressInput,
		tags: ['autodocs'],
		argTypes: {
			value: { control: 'text' }
		},
		args: {
			onChange: fn()
		}
	});
</script>

<!-- More on writing stories with args: https://storybook.js.org/docs/writing-stories/args -->
<Story name="Primary" args={{ value: 'not set' }} />
