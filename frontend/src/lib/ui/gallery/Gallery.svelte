<script lang="ts">
	import FALLBACK from '$lib/assets/tree.jpg';
	import { ExpandIcon } from '$lib/icons';
	import { hooks } from './hooks';
	import { longtap } from '$lib/utils/longtap';
	import { menuState } from '$lib/stores/treeMenu';
	import { onMount } from 'svelte';
	import { routes } from '$lib/routes';

	const { id } = $props<{ id: string }>();
	const { loading, error, files, reload, added_at, handleExpand } = hooks();

	let ref: HTMLDivElement;

	// Show context menu on image long tap
	onMount(() => longtap(ref, () => menuState.update(() => true), 500));

	$effect(() => reload(id));
</script>

<div class="gallery" bind:this={ref}>
	{#if $loading}
		<!-- Loading files -->
	{:else if $error}
		<p>{$error}</p>
	{:else}
		<div class="slides">
			{#each $files as file, idx}
				<div>
					<img src={routes.file(file.small_id)} alt="See how good is this tree." />
					<div class="imgno">{idx + 1}/{$files.length}</div>
					{#if file.added_at}
						<div class="date">{added_at(file)}</div>
					{/if}
					<button class="expand" onclick={() => handleExpand(file)}>
						<ExpandIcon />
					</button>
				</div>
			{:else}
				<div>
					<img src={FALLBACK} alt="no photos of this tree" />
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.gallery {
		position: relative;
		box-sizing: border-box;
		width: 100%;
		aspect-ratio: 1/1;
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

	.slides {
		display: flex;
		max-width: 100%;
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

		.date {
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
</style>
