<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import HeightInput from './HeightInput.svelte';
	import { fn } from '@storybook/test';

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: 'UI/HeightInput',
		component: HeightInput,
		argTypes: {
			value: { control: 'text' },
			autofocus: { control: 'boolean' }
		},
		args: {
			onChange: fn()
		}
	});
</script>

<Story name="Primary" args={{ value: 1.23 }} />
