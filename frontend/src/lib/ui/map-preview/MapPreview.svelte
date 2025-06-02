<script lang="ts">
	import { GalleryPreview, Buttons, Button, TreeContextMenu } from '$lib/ui';
	import { CloseIcon, ShareIcon, SettingsIcon } from '$lib/icons';
	import { routes } from '$lib/routes';
	import { formatSpecies, shortDetails } from '$lib/utils/trees';
	import { handleShareTree } from '$lib/hooks';
	import { locale } from '$lib/locale';
	import { hook } from './hooks';
	import '$lib/styles/variables.css';

	const { id } = $props<{ id: string }>();
	const { visible, error, tree, handleClose, handleContextMenu, reload } = hook();

	$effect(() => reload(id));
</script>

{#if $visible}
	<div class="preview">
		{#if $error}
			<p>{$error}</p>
		{:else if $tree}
			<div class="block">
				<div class="header">
					<div class="title">{formatSpecies($tree.species)}</div>
					<button class="close" onclick={handleClose}><CloseIcon /></button>
				</div>

				<div class="props">
					{#if $tree.address}
						<div class="line">{$tree.address}</div>
					{/if}
					<div class="line">{shortDetails($tree)}</div>
				</div>
			</div>

			<GalleryPreview id={$tree.id} />

			<Buttons>
				<Button link={routes.treeDetails($tree.id)}>{locale.mapPreviewDetails()}</Button>
				<Button type="secondary" onClick={() => handleShareTree($tree.id)} square
					><ShareIcon /></Button
				>
				<Button type="secondary" onClick={handleContextMenu} square><SettingsIcon /></Button>
			</Buttons>

			<TreeContextMenu id={$tree.id} />
		{/if}
	</div>
{/if}

<style>
	.preview {
		display: flex;
		flex-direction: column;
		gap: var(--gap);

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

			.close {
				flex-basis: 30px;
				flex-grow: 0;
				flex-shrink: 0;

				width: 30px;
				height: 30px;
				cursor: pointer;

				background-color: transparent;
				border: none;
				color: #fff;
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

			.line {
				white-space: nowrap;
				overflow: hidden;
				text-overflow: ellipsis;
			}
		}
	}

	/* Make it narrow on large mobile devices */
	@media (min-width: 600px) and (max-width: 1023px) {
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

			.title {
				display: flex;
				flex-direction: column;
				gap: var(--gap);
				margin-bottom: var(--gap);
			}
		}
	}
</style>
