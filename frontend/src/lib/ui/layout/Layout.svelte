<script lang="ts">
	import { MobileSidebar } from '$lib/ui';
	import MapPreview from '$lib/components/layout/MapPreview.svelte';
	import LeftSidebar from '$lib/components/layout/LeftSidebar.svelte';
	import MobileNav from '$lib/components/MobileNav.svelte';

	const { children } = $props();
</script>

<div class="layout">
	<aside class="left">
		<LeftSidebar />
		<MapPreview />
	</aside>

	<main>
		<article>
			{@render children()}
		</article>

		<MobileNav />
	</main>
</div>

<MobileSidebar />

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

		/**
		 * We have the navigation bar at the bottom which is fixed positioned.
		 * This adds room for that bar.
		 */
		article {
			padding-bottom: 50px;
		}
	}

	/**
	 * Desktop styles.  Navigation bar padding not needed.
	 */
	@media (min-width: 1024px) {
		.layout {
			article {
				padding-bottom: 0px;
			}
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
		z-index: 2;
	}

	main {
		z-index: 1;
	}
</style>
