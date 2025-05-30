import type { Preview } from '@storybook/svelte';
import '$lib/styles/variables.css';
import '$lib/styles/fonts.css';
import '$lib/styles/colors.css';

const preview: Preview = {
	parameters: {
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/i
			}
		}
	}
};

export default preview;
