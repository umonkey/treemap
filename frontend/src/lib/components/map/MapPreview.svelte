<script lang="ts">
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import GalleryPreview from '$lib/components/map/GalleryPreview.svelte';
	import { routes } from '$lib/routes';
	import { shortDetails, formatSpecies } from '$lib/utils/trees';

	export let tree;
	export let onClose;
</script>

{#if tree}
	<div class="preview">
		<div class="header">
			<div class="title">
				<a href={routes.treeDetails(tree.id)}>{formatSpecies(tree.species)}</a>
				{#if tree.address}
					&middot; <a href={routes.searchAddress(tree.address)}>{tree.address}</a>{/if}
			</div>
			<button class="close" on:click={onClose}><CloseIcon /></button>
		</div>
		<div class="props">{shortDetails(tree)}</div>
		<GalleryPreview {tree} />
	</div>
{/if}

<style>
	.preview {
		padding: var(--gap);
		line-height: 1.5em;
		z-index: var(--z-map-preview);

		width: 100%;
		box-sizing: border-box;
		background-color: var(--form-background);
		border-top-left-radius: 8px;
		border-top-right-radius: 8px;

		padding: var(--gap);
	}

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

	@media (max-width: 480px) {
		.preview {
			position: fixed;
			bottom: 0px;
		}
	}

	@media (min-width: 481px) {
		.preview {
			position: absolute;
			bottom: 0;
			left: 0;
		}
	}
</style>
