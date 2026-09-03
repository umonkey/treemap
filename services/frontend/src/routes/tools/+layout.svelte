<script lang="ts">
	import LeftSidebar from '$lib/components/layout/LeftSidebar.svelte';
	import MobileNav from '$lib/components/layout/MobileNav.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';

	const { children } = $props();
</script>

<div class="tools-layout">
	<div class="content">
		{@render children()}
	</div>
</div>

<LeftSidebar />
<MobileNav />
<TreeContextMenu />

<style>
	.tools-layout {
		min-height: 100dvh;
		background-color: var(--color-dialog-background);
		box-sizing: border-box;
	}

	.content {
		max-width: 800px;
		margin: 0 auto;
		padding: var(--gap);
		padding-bottom: calc(var(--gap) + env(safe-area-inset-bottom));
	}

	@media screen and (min-width: 1024px) {
		.tools-layout {
			margin-left: 300px;
		}
	}

	@media screen and (max-width: 1023px) {
		.content {
			padding-bottom: calc(var(--bottom-nav-height) + var(--gap) + env(safe-area-inset-bottom));
		}
	}
</style>
