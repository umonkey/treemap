<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { AddRowForm } from '$lib/ui';
	import { fn } from '@storybook/test';

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: 'Forms/AddRowForm',
		component: AddRowForm,
		argTypes: {
			start: { control: 'object' },
			end: { control: 'object' }
		},
		args: {
			start: { lat: 40.18113365329371, lng: 44.51390594244004 },
			end: { lat: 40.18148816925248, lng: 44.51421707868577 },
			onChange: fn()
		}
	});
</script>

<!-- More on writing stories with args: https://storybook.js.org/docs/writing-stories/args -->
<Story name="Primary" />
