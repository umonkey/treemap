<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { fn } from '@storybook/test';
	import RowSizeInput from './RowSizeInput.svelte';

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: 'UI/RowSizeInput',
		component: RowSizeInput,
		argTypes: {
			value: { control: 'number' },
			autofocus: { control: 'boolean' }
		},
		args: {
			distance: 61,
			onChange: fn()
		}
	});
</script>

<Story name="Primary" args={{ value: 3 }} />
