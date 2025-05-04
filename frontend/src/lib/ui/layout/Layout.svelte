<script lang="ts">
	import { LeftSidebar, RightSidebar, MobileSidebar } from '$lib/ui';
	import MobileNav from '$lib/components/MobileNav.svelte';

	const { children } = $props();
</script>

<div class="layout">
	<LeftSidebar />

	<main>
		<article>
			{@render children()}
		</article>

		<MobileNav />
	</main>

	<RightSidebar />
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
	@media (min-width: 768px) {
		.layout {
			article {
				padding-bottom: 0px;
			}
		}
	}
</style>
