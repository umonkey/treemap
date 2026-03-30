<script lang="ts">
	import MapPreview from '$lib/components/layout/MapPreview.svelte';
	import LeftSidebar from '$lib/components/layout/LeftSidebar.svelte';
	import MobileNav from '$lib/components/layout/MobileNav.svelte';
	import MapLibre from '$lib/components/map/MapLibre.svelte';
	import AddTree from '$lib/components/map/AddTree.svelte';
	import AddRow from '$lib/components/map/AddRow.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';

	const { children } = $props();
</script>

<div class="layout">
	<aside class="left">
		<LeftSidebar />
		<MapPreview />
	</aside>

	<main>
		<MapLibre>
			<AddTree />
			<AddRow />
		</MapLibre>
	</main>
</div>

{@render children()}
<MobileNav />
<TreeContextMenu />

<style>
	/**
	 * Default styles for phones, aka mobile first.
	 */
	.layout {
		display: flex;
		flex-direction: row;
		margin: 0 auto;
		width: 100%;
		box-sizing: border-box;

		main {
			width: 100%;
			min-height: 100dvh;
		}
	}

	/** Hide the side bar on mobile. **/
	aside {
		flex-basis: 0;
	}

	@media screen and (min-width: 1024px) {
		aside {
			display: block;
			flex-basis: 300px;
			flex-shrink: 0;
			flex-grow: 0;
		}
	}

	/** Make sure tree preview and menus overlap the map. **/
	aside {
		z-index: var(--z-sidebar);
	}

	main {
		z-index: var(--z-map);
	}

	@media screen and (max-width: 600px) {
		aside {
			z-index: var(--z-mobile-sidebar);
		}
	}
</style>
