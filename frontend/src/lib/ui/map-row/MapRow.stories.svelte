<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { Map, MapRow } from '$lib/ui';
	import { DEFAULT_MAP_CENTER } from '$lib/constants';

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

{#snippet wrapper(args)}
	<div class="wrapper">
		<Map center={DEFAULT_MAP_CENTER}>
			<MapRow {...args} />
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
