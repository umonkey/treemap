<script lang="ts" module>
	/* eslint-disable @typescript-eslint/no-explicit-any */
	import { DEFAULT_MAP_CENTER } from '$lib/constants';
	import { Map, MapRow } from '$lib/ui';
	import { defineMeta } from '@storybook/addon-svelte-csf';

	const { Story } = defineMeta({
		title: 'Map/MapRow',
		component: MapRow,
		parameters: {
			layout: 'fullscreen'
		},
		argTypes: {
			start: {
				control: 'object',
				description: 'Start point of the row'
			},
			end: {
				control: 'object',
				description: 'End point of the row'
			},
			count: {
				control: 'number',
				description: 'Number of segments in the row'
			}
		},
		args: {
			start: { lat: 40.18113365329371, lng: 44.51390594244004 },
			end: { lat: 40.18148816925248, lng: 44.51421707868577 },
			count: 5
		}
	});
</script>

{#snippet wrapper(args: any)}
	<div class="wrapper">
		<Map center={DEFAULT_MAP_CENTER}>
			<MapRow {...args} />
		</Map>
	</div>
{/snippet}

<Story name="Primary" args={{}} template={wrapper} />

<style>
	.wrapper {
		padding: 0;

		width: 100vw;
		height: 100vh;

		box-sizing: border-box;
	}
</style>
