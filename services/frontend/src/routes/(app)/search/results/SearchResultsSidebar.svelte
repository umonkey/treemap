<script lang="ts">
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import SpinnerIcon from '$lib/icons/SpinnerIcon.svelte';
	import { routes } from '$lib/routes';
	import { formatSpecies, formatState, shortDetails } from '$lib/utils/trees';
	import { locale } from './lang';
	import { SearchResultsSidebarLogic } from './SearchResultsSidebar.svelte.ts';
	import '$lib/styles/variables.css';

	const { query }: { query: string } = $props();

	const componentState = new SearchResultsSidebarLogic();

	$effect(() => {
		return componentState.init(query);
	});

	$effect(() => {
		componentState.reload(query);
	});
</script>

<div class="preview" class:loading={componentState.loading}>
	<div class="header">
		<div class="title">
			{#if componentState.query && !componentState.loading}
				{locale.titleWithCount(componentState.trees.length)}
			{:else}
				{locale.title()}
			{/if}
		</div>
		<button class="close" aria-label={locale.close()} onclick={componentState.handleClose}>
			<CloseIcon />
		</button>
	</div>

	<div class="props">
		{#if componentState.loading}
			<div class="loading-state">
				<SpinnerIcon />
				<span>{locale.loading()}</span>
			</div>
		{:else if !componentState.query}
			<div class="line">
				<div class="value count">
					{locale.emptyQuery()}
				</div>
			</div>
		{:else if componentState.trees.length === 0}
			<div class="line">
				<div class="value count">
					{locale.noResults()}
				</div>
			</div>
		{/if}
	</div>

	{#if !componentState.loading && componentState.trees.length > 0}
		<ul class="trees">
			{#each componentState.trees as tree (tree.id)}
				<li class="tree-item">
					<a
						href={routes.mapPreview(tree.id)}
						class="state-{tree.state}"
						onclick={() => componentState.handleTreeClick(tree)}
					>
						<div class="primary">
							{formatSpecies(tree.species)}
							<span class="state-label">{formatState(tree.state)}</span>
						</div>
						<div class="secondary">
							{shortDetails(tree)}
						</div>
					</a>
				</li>
			{/each}
		</ul>
	{/if}
</div>

<style>
	.preview {
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
			gap: var(--gap);
			height: 100%;
			min-height: 60px;

			:global(svg) {
				width: 20px;
				height: 20px;
			}
		}

		.header {
			display: flex;
			flex-direction: row;
			align-items: center;

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

				&:hover {
					opacity: 1;
				}
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

				.value {
					word-break: break-word;
					overflow: hidden;
					text-overflow: ellipsis;
				}
			}
		}

		ul.trees {
			flex: 1 1 auto;
			min-height: 0;
			overflow-y: auto;
			margin: 0;
			padding: 0;
			list-style: none;
			display: flex;
			flex-direction: column;
			gap: var(--gap);

			.tree-item {
				a {
					display: flex;
					flex-direction: column;
					gap: 2px;
					color: inherit;
					text-decoration: none;
					padding: 6px 8px;
					border-radius: 4px;
					border-left: 4px solid transparent;
					background-color: light-dark(rgba(0, 0, 0, 0.04), rgba(255, 255, 255, 0.06));

					&.state-alive,
					&.state-healthy {
						border-left-color: green;
					}

					&.state-dead {
						border-left-color: gray;
					}

					&.state-stump {
						border-left-color: black;
					}

					&.state-gone,
					&.state-replaced {
						border-left-color: transparent;
					}

					&:hover {
						background-color: light-dark(rgba(0, 0, 0, 0.08), rgba(255, 255, 255, 0.12));
					}

					.primary {
						font-weight: 500;
						white-space: nowrap;
						overflow: hidden;
						text-overflow: ellipsis;

						.state-label {
							font-size: 80%;
							opacity: 0.5;
						}
					}

					.secondary {
						font-size: 85%;
						opacity: 0.7;
						white-space: nowrap;
						overflow: hidden;
						text-overflow: ellipsis;
					}
				}
			}
		}
	}

	/* Make it narrow on large mobile devices */
	@media (min-width: 600px) and (max-width: 1023px) {
		.preview {
			width: 500px;
			left: calc((100vw - 500px) / 2);
			border-width: 0;
		}
	}

	/**
	 * This is for desktops.
	 */
	@media (min-width: 1024px) {
		.preview {
			position: fixed;
			top: 0;
			left: 0;
			width: 300px;
			height: 100vh;
			border-radius: 0px;
			border-left: 1px solid var(--sep-color);

			.title {
				display: flex;
				flex-direction: column;
				gap: var(--gap);
			}
		}
	}

	/** On mobile, positioned at bottom **/
	@media screen and (max-width: 1023px) {
		.preview {
			position: fixed;
			bottom: var(--bottom-nav-height);
			height: auto;
			max-height: 80dvh;
			padding-bottom: var(--gap);
			border-width: 0;

			animation: slideUp 0.2s ease-out;
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
