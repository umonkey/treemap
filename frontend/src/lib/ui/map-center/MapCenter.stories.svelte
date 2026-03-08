<script lang="ts" module>
	import { DEFAULT_MAP_CENTER } from '$lib/constants';
	import { Map, MapCenter } from '$lib/ui';
	import { defineMeta } from '@storybook/addon-svelte-csf';

	const { Story } = defineMeta({
		title: 'Map/MapCenter',
		component: MapCenter,
		parameters: {
			layout: 'fullscreen'
		}
	});
</script>

{#snippet wrapper()}
	<div class="wrapper">
		<Map center={DEFAULT_MAP_CENTER}>
			<MapCenter />
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
