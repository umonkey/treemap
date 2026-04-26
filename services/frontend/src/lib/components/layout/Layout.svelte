<script lang="ts">
	import LeftSidebar from '$lib/components/layout/LeftSidebar.svelte';
	import MapPreview from '$lib/components/layout/MapPreview.svelte';
	import MobileNav from '$lib/components/layout/MobileNav.svelte';
	import AddRow from '$lib/components/map/AddRow.svelte';
	import AddTree from '$lib/components/map/AddTree.svelte';
	import MapLibre from '$lib/components/map/MapLibre.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { page } from '$app/stores';
	import { searchStore } from '$lib/stores/searchStore';

	const { children } = $props();

	const q = $derived($page.url.searchParams.get('q'));

	$effect(() => {
		if (q !== undefined && q !== null) {
			searchStore.set(q);
		}
	});
</script>

<div class="layout">
	<aside class="left"></aside>

	<main>
		<MapLibre>
			<AddTree />
			<AddRow />
		</MapLibre>
	</main>
</div>

{@render children()}
<MapPreview />
<LeftSidebar />
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

	main {
		z-index: var(--z-map);
	}
</style>
