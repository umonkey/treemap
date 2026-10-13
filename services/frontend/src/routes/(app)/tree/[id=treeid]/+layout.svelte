<script lang="ts">
	import { setContext } from 'svelte';
	import { page } from '$app/state';
	import { mapBottomPadding } from '$lib/components/map/mapBottomPadding';
	import SidebarSearchInput from '$lib/components/search/SidebarSearchInput.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import SpinnerIcon from '$lib/icons/SpinnerIcon.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { formatSpecies } from '$lib/utils/trees';
	import TreeTabs from './components/TreeTabs.svelte';
	import { TREE_CONTEXT_KEY, TreeLayoutState } from './layout.svelte.ts';

	const { children } = $props();

	const state = new TreeLayoutState();
	setContext(TREE_CONTEXT_KEY, state);

	const id = $derived(page.params.id as string);

	// The sidebar is wide on the content tabs, but narrow where the content is:
	// the preview page and the move view.
	const narrowPaths = $derived([routes.treeDetails(id), routes.treeMove(id)]);
	const wide = $derived(!narrowPaths.includes(page.url.pathname));

	// The sidebar close button returns to the map from the tree root, and to the
	// tree root from any other tree tab.
	const isRoot = $derived(page.url.pathname === routes.treeDetails(id));

	$effect(() => {
		return state.init();
	});

	$effect(() => {
		if (id) state.reload(id);
	});

	const title = $derived(
		state.tree
			? `${formatSpecies(state.tree.species)} — ${state.tree.address ?? 'address unknown'} — ${locale.appTitle()}`
			: `${locale.treeShortTitle()} — ${locale.appTitle()}`
	);
</script>

<svelte:head>
	<title>{title}</title>
</svelte:head>

<div class="tree" class:wide class:loading={state.loading} use:mapBottomPadding>
	{#if state.tree}
		{@const tree = state.tree}

		<div class="search">
			<SidebarSearchInput
				value={state.query}
				onInput={state.handleInput}
				onSearch={state.handleSearch}
				onClose={() => state.handleClose(isRoot)}
			/>
		</div>

		<div class="header">
			<div class="title">{formatSpecies(tree.species)}</div>
			<button
				class="close"
				aria-label={locale.editCancel()}
				onclick={() => state.handleClose(isRoot)}><CloseIcon /></button
			>
		</div>

		<div class="subtitle">
			<div class="addr">{tree.address ?? 'Address unknown'}</div>
			<button
				class="id"
				type="button"
				title={locale.copyId()}
				aria-label={locale.copyId()}
				onclick={state.copyId}>#{tree.id}</button
			>
		</div>

		<TreeTabs {tree} />
	{:else if state.loading}
		<div class="loading-state"><SpinnerIcon /></div>
	{/if}

	<div class="content">
		{@render children()}
	</div>
</div>

<style>
	.tree {
		z-index: 2;

		display: flex;
		flex-direction: column;
		gap: var(--gap);

		padding: var(--gap);
		line-height: 1.5em;

		/* Default positioning for small phones */
		position: fixed;
		bottom: 0px;

		width: 100%;
		min-height: 132px;
		box-sizing: border-box;
		background-color: var(--map-menu-background);
		border-top-left-radius: 8px;
		border-top-right-radius: 8px;
		border-right: 1px solid var(--color-dialog-border);

		.loading-state {
			display: flex;
			align-items: center;
			justify-content: center;
			height: 100%;
			min-height: 100px;

			:global(svg) {
				width: 20px;
				height: 20px;
			}
		}

		/* The search row is desktop-only. */
		.search {
			display: none;
		}

		.header {
			display: flex;
			flex-direction: row;

			.close {
				flex-basis: 30px;
				flex-grow: 0;
				flex-shrink: 0;

				width: 30px;
				height: 30px;
				cursor: pointer;

				background-color: transparent;
				border: none;
				color: light-dark(black, white);
				opacity: 0.5;
			}
		}

		.title {
			flex-grow: 1;
			flex-shrink: 1;
			font-size: 120%;
			line-height: 30px;

			white-space: nowrap;
			overflow: hidden;
			text-overflow: ellipsis;
		}
	}

	.content {
		display: flex;
		flex-direction: column;
		gap: var(--gap);
		min-height: 0;
		scrollbar-width: none;
	}

	.subtitle {
		display: flex;
		flex-direction: row;
		gap: 1rem;
		opacity: 0.5;

		.addr {
			flex: 1 1;
		}

		.id {
			opacity: 0.5;
			margin: 0;
			padding: 0;
			border: none;
			background: none;
			color: inherit;
			font: inherit;
			line-height: inherit;
			cursor: pointer;
		}
	}

	/* Make it narrow on large mobile devices */
	@media (min-width: 600px) and (max-width: 1023px) {
		.tree {
			width: 500px;
			left: calc((100vw - 500px) / 2);
			border-width: 0;
		}
	}

	/**
	 * This is for desktops.
	 */
	@media (min-width: 1024px) {
		.tree {
			position: fixed;
			top: 0;
			left: 0;
			width: var(--sidebar-width-desktop);
			height: 100vh;
			border-radius: 0px;
			border-left: 1px solid var(--sep-color);

			&.wide {
				width: var(--sidebar-width-desktop-wide);
			}

			.title {
				display: flex;
				flex-direction: column;
				gap: var(--gap);
				margin-bottom: var(--gap);
			}

			/* The close button moves to the search row on desktop. */
			.header .close {
				display: none;
			}

			.search {
				display: block;
			}
		}

		.content {
			overflow-y: auto;
		}
	}

	/**
	 * On mobile, the panel is collapsed by default: it hugs whatever chrome it
	 * holds. Expanding reveals the content, letting the panel grow with it, up
	 * to 80dvh; beyond that the content scrolls.
	 **/
	@media screen and (max-width: 1023px) {
		.tree {
			position: fixed;
			bottom: var(--bottom-nav-height);
			max-height: 80dvh;
			padding-bottom: var(--gap);
			border-width: 0;
			overflow: hidden;

			animation: slideUp 0.2s ease-out;

			.content {
				max-height: 0;
				overflow: hidden;
				transition: max-height 0.2s ease-in-out;
			}

			.content {
				max-height: 80dvh;
				overflow-y: auto;
			}
		}
	}

	@keyframes slideUp {
		from {
			transform: translateY(100%);
		}
		to {
			transform: translateY(0);
		}
	}
</style>
