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

	@media (max-width: 480px) {
		.layout {
			article {
				padding-bottom: 50px;
			}
		}
	}
</style>
