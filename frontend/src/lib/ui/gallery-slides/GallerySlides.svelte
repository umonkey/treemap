<script lang="ts">
	import FALLBACK from '$lib/assets/tree.jpg';
	import { ExpandIcon, LeftButtonIcon, RightButtonIcon } from '$lib/icons';
	import type { IGalleryItem } from '$lib/types';
	import { hooks } from './hooks';

	const { slides } = $props<{
		slides: IGalleryItem[];
	}>();

	const { handleExpand, handleLeft, handleRight, element } = hooks();
</script>

<div class="slides" bind:this={$element}>
	{#each slides as item, idx}
		<div>
			<img src={item.small} alt="See how good is this tree." />

			{#if idx > 0}
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<div class="nav left" onclick={handleLeft}>
					<LeftButtonIcon />
				</div>
			{/if}

			{#if idx < slides.length - 1}
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<div class="nav right" onclick={handleRight}>
					<RightButtonIcon />
				</div>
			{/if}

			<div class="imgno">{idx + 1}/{slides.length}</div>
			{#if item.label}
				<div class="label">{item.label}</div>
			{/if}
			<button class="expand" onclick={() => handleExpand(item.large)}>
				<ExpandIcon />
			</button>
		</div>
	{:else}
		<div>
			<img src={FALLBACK} alt="no photos of this tree" />
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
			}
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

			:global(svg) {
				width: 30px;
				height: 30px;
				opacity: 0.5;
			}
		}

		.left {
			left: 0;
			right: 33%;
			justify-content: left;
		}

		.right {
			left: 66%;
			right: 0;
			justify-content: right;
		}
	}
</style>
