<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { MapRowPreview } from '$lib/ui';

	const { Story } = defineMeta({
		title: 'Map/MapRowPreview',
		component: MapRowPreview,
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

<Story name="Primary" />
