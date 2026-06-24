<script lang="ts">
	import LeftSidebar from '$lib/components/layout/LeftSidebar.svelte';
	import MobileNav from '$lib/components/layout/MobileNav.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';

	const { children } = $props();
</script>

<AuthWrapper>
	<div class="admin-layout">
		<div class="content">
			{@render children()}
		</div>
	</div>
</AuthWrapper>

<LeftSidebar />
<MobileNav />
<TreeContextMenu />

<style>
	.admin-layout {
		min-height: 100dvh;
		background-color: var(--color-dialog-background);
		box-sizing: border-box;

		:global(h1) {
			font-weight: 400;
			font-size: 2rem;
			line-height: 1.125;
		}

		:global(ul li) {
			margin: 0.5rem 0;
		}
	}

	.content {
		max-width: 1000px;
		margin: 0 auto;
		padding: var(--gap);
		padding-bottom: calc(var(--gap) + env(safe-area-inset-bottom));
	}

	@media screen and (min-width: 1024px) {
		.admin-layout {
			margin-left: 300px;
		}
	}

	@media screen and (max-width: 1023px) {
		.content {
			padding-bottom: calc(var(--bottom-nav-height) + var(--gap) + env(safe-area-inset-bottom));
		}
	}
</style>
