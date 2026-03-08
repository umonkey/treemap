<script lang="ts">
	import { Buttons, Button } from '$lib/ui';
	import GalleryPreview from '$lib/components/photos/GalleryPreview.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { CloseIcon, ShareIcon, SettingsIcon } from '$lib/icons';
	import LocationIcon from '$lib/icons/LocationIcon.svelte';
	import BatteryIcon from '$lib/icons/BatteryIcon.svelte';
	import TagIcon from '$lib/icons/TagIcon.svelte';
	import Observations from '$lib/components/observation/Observations.svelte';
	import Comment from '$lib/components/tree/Comment.svelte';
	import { routes } from '$lib/routes';
	import { formatSpecies, formatState, shortDetails } from '$lib/utils/trees';
	import { handleShareTree } from '$lib/hooks';
	import { locale } from '$lib/locale';
	import { hook } from './MapPreview';
	import '$lib/styles/variables.css';

	const {
		visible,
		expand,
		error,
		tree,
		observations,
		comments,
		handleClose,
		handleContextMenu,
		toggleExpand
	} = hook();
</script>

{#if $visible}
	<div class="preview" class:expand={!!$expand}>
		{#if $error}
			<p>{$error}</p>
		{:else if $tree}
			<div class="header">
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<div class="title" onclick={toggleExpand}>{formatSpecies($tree.species)}</div>
				<button class="close" onclick={handleClose}><CloseIcon /></button>
			</div>

			<div class="props">
				{#if $tree.address}
					<div class="line">
						<div class="icon">
							<LocationIcon />
						</div>
						<div class="value">{$tree.address}</div>
					</div>
				{/if}
				<div class="line">
					<div class="icon">
						<TagIcon />
					</div>
					<div class="value">{shortDetails($tree)}</div>
				</div>
				<div class="line">
					<div class="icon">
						<BatteryIcon />
					</div>
					<div class="value">{formatState($tree.state)}</div>
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

			<div class="extras">
				<Observations observation={$observations} />

				{#each $comments as comment}
					<Comment {comment} />
				{/each}
			</div>

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
			left: 0;
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

	/** On mobile, extrass need expansion. **/
	@media screen and (max-width: 600px) {
		.preview {
			height: 266px;
			transition: height 0.2s ease-in-out;

			.extras {
				display: none;
			}

			&.expand {
				height: 80vh;

				.extras {
					margin-top: var(--gap);
					overflow-y: scroll;

					display: flex;
					flex-direction: column;
					gap: var(--gap);
				}
			}
		}
	}
</style>
