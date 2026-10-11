<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { settingsState } from '$lib/stores/settings.svelte';
	import MapLibre from './MapLibre.svelte';

	// The map is gated until settings load (normally triggered by the root
	// layout). Storybook does not render that layout, so mark settings as
	// loaded here to let the fallback bbox view render.
	settingsState.loaded = true;

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
		<MapLibre />
	</div>
{/snippet}

<Story name="Default" template={wrapper} />

<style>
	.wrapper {
		width: 100vw;
		height: 100vh;
	}
</style>
