<script lang="ts">
	import { setContext } from 'svelte';
	import { page } from '$app/state';
	import { mapBottomPadding } from '$lib/components/map/mapBottomPadding';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import LocationIcon from '$lib/icons/LocationIcon.svelte';
	import SpinnerIcon from '$lib/icons/SpinnerIcon.svelte';
	import { locale } from '$lib/locale';
	import { formatSpecies } from '$lib/utils/trees';
	import TreeTabs from './components/TreeTabs.svelte';
	import { TREE_CONTEXT_KEY, TreeLayoutState } from './layout.svelte.ts';

	const { children } = $props();

	const state = new TreeLayoutState();
	setContext(TREE_CONTEXT_KEY, state);

	const id = $derived(page.params.id as string);

	$effect(() => {
		return state.init();
	});

	$effect(() => {
		if (id) state.reload(id);
	});

	const active = $derived.by(() => {
		const path = page.url.pathname;
		if (path.endsWith('/preview')) return 'preview';
		if (path.endsWith('/360')) return '360';
		if (path.endsWith('/observations')) return 'observations';
		if (path.endsWith('/photos')) return 'photos';
		if (path.endsWith('/history')) return 'history';
		return 'details';
	});

	const title = $derived(
		state.tree
			? `${formatSpecies(state.tree.species)} — ${state.tree.address} — ${locale.appTitle()}`
			: `${locale.treeShortTitle()} — ${locale.appTitle()}`
	);
</script>

<svelte:head>
	<title>{title}</title>
</svelte:head>

<div class="tree" class:expand={!!state.expand} class:loading={state.loading} use:mapBottomPadding>
	{#if state.tree}
		{@const tree = state.tree}

		<div class="header">
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<div class="title" onclick={state.toggleExpand}>{formatSpecies(tree.species)}</div>
			<button class="close" aria-label={locale.editCancel()} onclick={state.handleClose}
				><CloseIcon /></button
			>
		</div>

		{#if tree.address}
			<div class="props">
				<div class="line">
					<div class="icon"><LocationIcon /></div>
					<div class="value">{tree.address}</div>
				</div>
			</div>
		{/if}

		<TreeTabs tree={tree.id} {active} />
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

		.props {
			opacity: 0.7;
			display: flex;
			flex-direction: column;
			gap: 5px;

			.line {
				white-space: nowrap;
				overflow: hidden;
				text-overflow: ellipsis;

				display: flex;
				flex-direction: row;
				align-items: center;
				gap: var(--gap);

				.icon {
					width: 20px;
					height: 20px;
				}
			}
		}
	}

	.content {
		display: flex;
		flex-direction: column;
		gap: var(--gap);
		min-height: 0;
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

			.title {
				display: flex;
				flex-direction: column;
				gap: var(--gap);
				margin-bottom: var(--gap);
			}
		}

		.content {
			overflow-y: auto;
		}
	}

	/** On mobile, the content needs expansion. **/
	@media screen and (max-width: 1023px) {
		.tree {
			position: fixed;
			bottom: var(--bottom-nav-height);
			height: calc(266px);
			padding-bottom: var(--gap);
			transition: height 0.2s ease-in-out;
			border-width: 0;
			overflow: hidden;

			animation: slideUp 0.2s ease-out;

			&.expand {
				height: 80dvh;

				.content {
					overflow-y: auto;
				}
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
