<script lang="ts" module>
	import { DEFAULT_MAP_CENTER } from '$lib/constants';
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import MapLibre from './MapLibre.svelte';

	const { Story } = defineMeta({
		title: 'Map/MapLibre',
		component: MapLibre,
		parameters: {
			layout: 'fullscreen'
		}
	});
</script>

{#snippet wrapper()}
	<div class="wrapper">
		<MapLibre center={DEFAULT_MAP_CENTER} />
	</div>
{/snippet}

<Story name="Default" template={wrapper} />

<style>
	.wrapper {
		width: 100vw;
		height: 100vh;
	}
</style>
