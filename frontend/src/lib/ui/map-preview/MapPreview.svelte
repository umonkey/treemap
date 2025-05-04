<script lang="ts">
	import { GalleryPreview } from '$lib/ui';
	import { CloseIcon } from '$lib/icons';
	import { routes } from '$lib/routes';
	import { formatSpecies, shortDetails } from '$lib/utils/trees';
	import { hook } from './hooks';
	import '$lib/styles/variables.css';

	const { id } = $props<{ id: string }>();
	const { visible, error, tree, handleClose, reload } = hook();

	$effect(() => reload(id));
</script>

{#if $visible}
	<div class="preview">
		{#if $error}
			<p>{$error}</p>
		{:else if $tree}
			<div class="header">
				<div class="title">
					<a href={routes.treeDetails($tree.id)}>{formatSpecies($tree.species)}</a>
					{#if $tree.address}
						&middot; {$tree.address}{/if}
				</div>
				<button class="close" onclick={handleClose}><CloseIcon /></button>
			</div>
			<div class="props">{shortDetails($tree)}</div>
			<GalleryPreview id={$tree.id} />
		{/if}
	</div>
{/if}

<style>
	.preview {
		padding: var(--gap);
		line-height: 1.5em;
		z-index: var(--z-map-preview);

		/* Default positioning for small phones */
		position: fixed;
		bottom: 0px;

		width: 100%;
		min-height: 132px;
		box-sizing: border-box;
		background-color: var(--form-background);
		border-top-left-radius: 8px;
		border-top-right-radius: 8px;

		padding: var(--gap);

		.header {
			display: flex;
			flex-direction: row;

			.title {
				flex-grow: 1;
				flex-shrink: 1;
			}

			.close {
				flex-basis: 30px;
				flex-grow: 0;
				flex-shrink: 0;

				width: 30px;
				height: 30px;
				cursor: pointer;

				background-color: transparent;
				border: none;
				color: var(--sep-color);
			}
		}
	}

	/* Make it narrow on large mobile devices */
	@media (min-width: 600px) {
		.preview {
			width: 500px;
			left: calc((100vw - 500px) / 2);
		}
	}

	/**
	 * This is for desktops.
	 */
	@media (min-width: 1024px) {
		.preview {
			position: fixed;
			top: 0;
			right: 0;
			width: 300px;
			height: 100vh;
			border-radius: 0px;
			border-left: 1px solid var(--sep-color);
		}
	}
</style>
