<script lang="ts" module>
	/* eslint-disable @typescript-eslint/no-explicit-any */
	import { DEFAULT_MAP_CENTER } from '$lib/constants';
	import { Map, MapAddTree, MapCenter } from '$lib/ui';
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'Map/MapAddTree',
		component: MapAddTree,
		parameters: {
			layout: 'fullscreen'
		},
		args: {
			onConfirm: fn()
		}
	});
</script>

{#snippet wrapper(args: any)}
	<div class="wrapper">
		<Map center={DEFAULT_MAP_CENTER}>
			<MapCenter />
			<MapAddTree {...args} />
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
