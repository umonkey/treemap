<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Button from './Button.svelte';
	import { fn } from '@storybook/test';

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: 'UI/Button',
		component: Button,
		argTypes: {
			label: { control: 'text' },
			type: {
				control: 'select',
				options: ['submit', 'button', 'reset', 'cancel', 'secondary', 'tertiary']
			},
			link: { control: 'text' },
			disabled: { control: 'boolean' }
		},
		args: {
			onClick: fn()
		}
	});
</script>

<Story name="Submit" args={{ label: 'Click Me', type: 'submit' }} />

<Story name="Disabled" args={{ label: 'Click Me', type: 'submit', disabled: true }} />

<Story name="Link" args={{ label: 'Click Me', link: 'https://www.google.com/', type: 'submit' }} />

<Story name="Reset" args={{ label: 'Click Me', type: 'reset' }} />

<Story name="Cancel" args={{ label: 'Click Me', type: 'cancel' }} />

<Story name="Secondary" args={{ label: 'Click Me', type: 'secondary' }} />

<Story name="Tretiary" args={{ label: 'Click Me', type: 'tretiary' }} />
