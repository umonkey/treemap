<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { Map, MapButton } from '$lib/ui';
	import { fn } from '@storybook/test';
	import { DEFAULT_MAP_CENTER } from '$lib/constants';
	import ICON1 from '$lib/assets/locate.svg';
	import ICON2 from '$lib/assets/crosshair.svg';
	import ICON3 from '$lib/assets/fullscreen.svg';

	const { Story } = defineMeta({
		title: 'Map/MapButton',
		component: MapButton,
		parameters: {
			layout: 'fullscreen'
		},
		argTypes: {
			position: {
				control: { type: 'select' },
				options: ['topleft', 'topright', 'bottomleft', 'bottomright'],
				description: 'Position of the button on the map'
			},
			icon: {
				control: { type: 'select' },
				options: [ICON1, ICON2, ICON3],
				description: 'Icon to display on the button'
			},
			active: {
				control: { type: 'boolean' },
				description: 'Whether the button is active or not'
			},
			onClick: {
				action: 'clicked',
				description: 'Function to call when the button is clicked'
			}
		},
		args: {
			position: 'topleft',
			icon: ICON1,
			active: false,
			onClick: fn()
		}
	});
</script>

{#snippet wrapper(args)}
	<div class="wrapper">
		<Map center={DEFAULT_MAP_CENTER}>
			<MapButton {...args} />
		</Map>
	</div>
{/snippet}

<Story name="Primary" args={{ simpleChild: true }} template={wrapper} />

<style>
	.wrapper {
		padding: 0;

		width: 100vw;
		height: 100vh;

		box-sizing: border-box;
	}
</style>
