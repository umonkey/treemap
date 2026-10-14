<script lang="ts">
	import ToolsSidebar from '$lib/components/tools/ToolsSidebar.svelte';
	import MobileNav from '$lib/components/layout/MobileNav.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';

	const { children } = $props();
</script>

<div class="tools-layout">
	<div class="content">
		{@render children()}
	</div>
</div>

<ToolsSidebar />
<MobileNav />
<TreeContextMenu />

<style>
	.tools-layout {
		min-height: 100dvh;
		background-color: var(--color-dialog-background);
		box-sizing: border-box;

		.content {
			:global(h1) {
				font-weight: 400;
				font-size: 2rem;
				margin: 0.5rem 0;
				opacity: 0.8;
			}

			:global(h2) {
				font-weight: 400;
				font-size: 1.5rem;
				margin: 2rem 0 0.5rem;
				opacity: 0.8;
			}
		}
	}

	.content {
		max-width: 800px;
		margin: 0 auto;
		padding: var(--gap);
		padding-bottom: calc(var(--gap) + env(safe-area-inset-bottom));
		overflow: scroll;
	}

	@media screen and (min-width: 1024px) {
		.tools-layout {
			margin-left: var(--sidebar-width-desktop);
		}
	}

	@media screen and (max-width: 1023px) {
		.content {
			padding-bottom: calc(var(--bottom-nav-height) + var(--gap) + env(safe-area-inset-bottom));
		}
	}
</style>
