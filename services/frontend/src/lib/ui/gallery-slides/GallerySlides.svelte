<script lang="ts">
	import FALLBACK from '$lib/assets/tree.jpg';
	import ExpandIcon from '$lib/icons/ExpandIcon.svelte';
	import LeftButtonIcon from '$lib/icons/LeftButton.svelte';
	import RightButtonIcon from '$lib/icons/RightButton.svelte';
	import type { IGalleryItem } from '$lib/types';
	import { galleryState } from './GallerySlides.svelte.ts';

	const { slides, initialImageId } = $props<{
		slides: IGalleryItem[];
		initialImageId?: string;
	}>();

	let contain = $state(false);
	const toggleContain = () => {
		contain = !contain;
	};

	$effect(() => {
		if (initialImageId && slides.length > 0) {
			galleryState.scrollToId(initialImageId, slides);
		}
	});
</script>

<div class="slides" class:contain bind:this={galleryState.element}>
	{#each slides as item, idx}
		<div>
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
			<img src={item.small} alt="See how good is this tree." onclick={toggleContain} />

			{#if idx > 0}
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<div class="nav left" onclick={galleryState.handleLeft}>
					<LeftButtonIcon />
				</div>
			{/if}

			{#if idx < slides.length - 1}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div class="nav right" onclick={galleryState.handleRight}>
					<RightButtonIcon />
				</div>
			{/if}

			<div class="imgno">{idx + 1}/{slides.length}</div>
			{#if item.label}
				<div class="label">{item.label}</div>
			{/if}
			<button class="expand" onclick={() => galleryState.handleExpand(item.large)}>
				<ExpandIcon />
			</button>
		</div>
	{:else}
		<div>
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
			<img src={FALLBACK} alt="no photos of this tree" onclick={toggleContain} />
		</div>
	{/each}
</div>

<style>
	.slides {
		display: flex;
		width: 100%;
		max-width: 600px;
		aspect-ratio: 1/1;

		overflow-x: auto;
		overflow-y: hidden;
		white-space: nowrap;

		scroll-snap-type: x mandatory;
		scrollbar-width: none;

		& > div {
			display: block;
			position: relative;
			scroll-snap-align: center;
			scroll-snap-stop: always;

			min-height: 100%;
			min-width: 100%;

			aspect-ratio: 1/1;
			text-align: center;
			align-content: center;
			scroll-snap-align: center;

			img {
				border: none;
				object-position: center;
				object-fit: cover;
				height: 100%;
				width: 100%;
				z-index: 1;
				cursor: pointer;
			}
		}

		&.contain img {
			object-fit: contain;
		}

		.label {
			position: absolute;
			bottom: var(--gap);
			left: var(--gap);
			background-color: rgba(0, 0, 0, 0.75);
			color: #fff;
			font-size: 14px;
			padding: 2px 4px;

			/* Prevent text overflow in case of really long username */
			max-width: calc(100% - 2 * var(--gap) - 8px);
			overflow: hidden;
			text-overflow: ellipsis;
		}

		.expand {
			width: 30px;
			height: 30px;
			border: none;
			padding: 0;
			position: absolute;
			bottom: var(--gap);
			right: var(--gap);
			cursor: pointer;
			background-color: var(--form-background);
			color: var(--text-color);
		}

		.imgno {
			position: absolute;
			right: var(--gap);
			top: var(--gap);
			background-color: rgba(0, 0, 0, 0.85);
			color: #fff;
			border-radius: 9px;
			font-size: 14px;
			padding: 2px 8px;
			z-index: 2;
		}

		.nav {
			position: absolute;
			top: 0;
			bottom: 0;

			display: flex;
			align-items: center;
			padding: 10px;
			box-sizing: border-box;
			cursor: pointer;

			color: #fff;
			outline: none;

			:global(svg) {
				width: 30px;
				height: 30px;
				opacity: 0.5;
			}
		}

		.left {
			left: 0;
			right: 66%;
			justify-content: left;
		}

		.right {
			left: 66%;
			right: 0;
			justify-content: right;
		}
	}
</style>
