<script lang="ts">
	import LeftSidebar from '$lib/components/layout/LeftSidebar.svelte';
	import MobileNav from '$lib/components/layout/MobileNav.svelte';
	import ContextMenu from '$lib/components/map/ContextMenu.svelte';
	import MapLibre from '$lib/components/map/MapLibre.svelte';
	import PhotoContextMenu from '$lib/components/photos/PhotoContextMenu.svelte';
	import UploadContextMenu from '$lib/components/photos/UploadContextMenu.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';

	import { page } from '$app/stores';
	import { searchStore } from '$lib/stores/searchStore';
	import { sidebarState } from '$lib/stores/sidebar.svelte';

	const { children } = $props();

	const q = $derived($page.url.searchParams.get('q'));

	$effect(() => {
		if (q !== undefined && q !== null) {
			searchStore.set(q);
		}
	});
</script>

<div class="layout">
	<aside class="left" class:wide={sidebarState.wide}></aside>

	<main>
		<MapLibre />
	</main>
</div>

{@render children()}
<LeftSidebar />
<MobileNav />
<MobileNav />
<TreeContextMenu />
<ContextMenu />
<PhotoContextMenu />
<UploadContextMenu />

<style>
	/**
	 * Default styles for phones, aka mobile first.
	 */
	.layout {
		display: flex;
		flex-direction: row;
		margin: 0 auto;
		width: 100%;
		height: 100dvh;
		overflow: hidden;
		box-sizing: border-box;

		main {
			width: 100%;
			height: 100%;
		}
	}

	/** Hide the side bar on mobile. **/
	aside {
		flex-basis: 0;
	}

	@media screen and (min-width: 1024px) {
		aside {
			display: block;
			flex-basis: var(--sidebar-width-desktop);
			flex-shrink: 0;
			flex-grow: 0;

			&.wide {
				flex-basis: var(--sidebar-width-desktop-wide);
			}
		}
	}

	main {
		z-index: var(--z-map);
	}
</style>
