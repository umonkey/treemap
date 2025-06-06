<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { Map, MapPin } from '$lib/ui';
	import { DEFAULT_MAP_CENTER } from '$lib/constants';

	const { Story } = defineMeta({
		title: 'Map/MapPin',
		component: MapPin,
		parameters: {
			layout: 'fullscreen'
		}
	});
</script>

{#snippet wrapper()}
	<div class="wrapper">
		<Map center={DEFAULT_MAP_CENTER}>
			<MapPin
				center={{
					lat: DEFAULT_MAP_CENTER[0],
					lng: DEFAULT_MAP_CENTER[1]
				}}
			/>
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
