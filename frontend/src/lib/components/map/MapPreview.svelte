<script lang="ts">
	import { GalleryPreview } from '$lib/ui';
	import { loadTree } from '$lib/hooks';
	import { CloseIcon } from '$lib/icons';
	import { routes } from '$lib/routes';
	import { formatSpecies, shortDetails } from '$lib/utils/trees';

	const { tree, onClose } = $props<{
		tree: string | null;
		onClose: () => void;
	}>();

	const { loading, data, error, reload } = loadTree();

	$effect(() => {
		if (tree) {
			(async () => await reload(tree))();
		}
	});
</script>

{#if tree}
	<div class="preview">
		{#if $error}
			<p>{$error}</p>
		{:else if $loading}
			<!-- a spinner -->
		{:else if $data}
			<div class="header">
				<div class="title">
					<a href={routes.treeDetails($data.id)}>{formatSpecies($data.species)}</a>
					{#if $data.address}
						&middot; {$data.address}{/if}
				</div>
				<button class="close" onclick={onClose}><CloseIcon /></button>
			</div>
			<div class="props">{shortDetails($data)}</div>
			<GalleryPreview tree={$data} />
		{/if}
	</div>
{/if}

<style>
	.preview {
		padding: var(--gap);
		line-height: 1.5em;
		z-index: var(--z-map-preview);

		width: 100%;
		min-height: 132px;
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
