<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { Map, MapCenter, MapAddTree } from '$lib/ui';
	import { fn } from '@storybook/test';
	import { DEFAULT_MAP_CENTER } from '$lib/constants';

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

{#snippet wrapper(args)}
	<div class="wrapper">
		<Map center={DEFAULT_MAP_CENTER}>
			<MapCenter />
			<MapAddTree {...args} />
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
