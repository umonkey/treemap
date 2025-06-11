<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { LocationTracker } from '$lib/ui';

	const { Story } = defineMeta({
		title: 'Map/LocationTracker',
		component: LocationTracker,
		parameters: {
			layout: 'fullscreen'
		}
	});
</script>

{#snippet wrapper()}
	<div class="wrapper">
		<LocationTracker />
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
