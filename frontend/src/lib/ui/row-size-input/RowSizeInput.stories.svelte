<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { RowSizeInput } from '$lib/ui';
	import { fn } from '@storybook/test';

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
