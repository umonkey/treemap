import type { Preview } from '@storybook/svelte';
import '@fontsource-variable/inter';
import '$lib/styles/variables.css';
import '$lib/styles/fonts.css';
import '$lib/styles/colors.css';
import './styles.css';
import { chromaticModes } from './modes';

const preview: Preview = {
	parameters: {
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/i
			}
		},
		chromatic: {
			modes: chromaticModes
		}
	}
};

export default preview;
