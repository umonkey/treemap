<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import HelpButton from './HelpButton.svelte';

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: 'UI/HelpButton',
		component: HelpButton,
		argTypes: {
			help: { control: 'text' }
		}
	});
</script>

<Story name="Primary" args={{ help: 'https://www.google.com/' }} />
