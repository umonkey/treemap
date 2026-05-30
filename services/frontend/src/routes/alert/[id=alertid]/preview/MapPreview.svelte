<script lang="ts">
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import ChatIcon from '$lib/icons/ChatIcon.svelte';
	import SpinnerIcon from '$lib/icons/SpinnerIcon.svelte';
	import { previewState } from './MapPreview.svelte.ts';
	import { formatDateTime } from '$lib/utils/strings';
	import LazyImage from '$lib/ui/lazy-image/LazyImage.svelte';
	import DefaultImage from '$lib/assets/tree.jpg';
	import { locale } from './lang';
	import '$lib/styles/variables.css';

	let { id } = $props<{ id: string }>();

	$effect(() => {
		return previewState.init();
	});

	$effect(() => {
		previewState.reload(id);
	});
</script>

<div class="preview" class:expand={!!previewState.expand} class:loading={previewState.loading}>
	{#if previewState.alert}
		{@const alert = previewState.alert}
		<div class="header">
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<div class="title" onclick={previewState.toggleExpand}>
				{locale.title(alert.id)}
			</div>
			<button class="close" onclick={previewState.handleClose}><CloseIcon /></button>
		</div>

		<div class="props">
			<div class="line">
				<div class="icon">
					<ChatIcon />
				</div>
				<div class="value">{alert.description || locale.noDescription()}</div>
			</div>
			<div class="line date">
				{formatDateTime(alert.created_at)}
			</div>
			<div class="line bot-report">
				<a
					href="https://t.me/kanach_yerevan_bot"
					target="_blank"
					rel="noopener noreferrer"
					class="value"
				>
					{locale.sendReport()}
				</a>
			</div>
		</div>

		{#if previewState.photos.length > 0}
			<div class="gallery">
				<div class="images">
					{#each previewState.photos as photo}
						<div class="tile">
							<a href={photo} target="_blank" rel="noopener noreferrer">
								<LazyImage src={photo} alt={locale.photoAlt()} fallback={DefaultImage} />
							</a>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	{:else if previewState.loading}
		<div class="loading-state">
			<SpinnerIcon />
		</div>
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

		position: fixed;
		bottom: 0px;

		width: 100%;
		min-height: 132px;
		box-sizing: border-box;
		background-color: var(--map-menu-background);
		border-top-left-radius: 8px;
		border-top-right-radius: 8px;
		border-right: 1px solid var(--color-dialog-border);

		padding: var(--gap);

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
				display: flex;
				flex-direction: row;
				align-items: flex-start;
				gap: var(--gap);

				&.date {
					font-size: 85%;
					margin-left: calc(20px + var(--gap));
				}

				&.bot-report {
					font-size: 85%;
					margin-left: calc(20px + var(--gap));

					a {
						color: var(--color-link);
						text-decoration: underline;
					}
				}

				.icon {
					width: 20px;
					height: 20px;
					flex-shrink: 0;
					margin-top: 2px;
				}

				.value {
					word-break: break-word;
				}
			}
		}

		.gallery {
			height: 75px;
			line-height: 75px;
			margin-top: 5px;

			.images {
				display: flex;
				flex-direction: row;
				gap: var(--gap);

				overflow-x: auto;
				scroll-snap-type: x mandatory;
				scrollbar-width: none;

				.tile {
					width: 75px;
					height: 75px;
					border-radius: 4px;
					overflow: hidden;
					flex-shrink: 0;
					flex-grow: 0;
					background-color: var(--color-dialog-header);

					a {
						display: block;
						width: 75px;
						height: 75px;
					}
				}
			}
		}
	}

	@media (min-width: 600px) and (max-width: 1023px) {
		.preview {
			width: 500px;
			left: calc((100vw - 500px) / 2);
			border-width: 0;
		}
	}

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
