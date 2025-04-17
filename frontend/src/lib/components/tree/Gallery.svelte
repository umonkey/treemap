<script lang="ts">
import { routes } from "$lib/routes";
import { menuState } from "$lib/stores/treeMenu";
import { getUser } from "$lib/stores/userStore";
import type { ITreeFile } from "$lib/types";
import { longtap } from "$lib/utils/longtap";
import { formatDate } from "$lib/utils/strings";
import { onMount } from "svelte";
import { get } from "svelte/store";

import ExpandIcon from "$lib/icons/ExpandIcon.svelte";

export const files: ITreeFile[] = [];
let ref: HTMLDivElement;

const added_at = (file: ITreeFile) => {
	if (!file.added_at || !file.added_by) {
		return "";
	}

	const user = get(getUser)(file.added_by);

	if (user === undefined) {
		return "";
	}

	const date = formatDate(file.added_at);
	return `${date} by ${user.name}`;
};

const onExpand = (file: ITreeFile) => {
	const url = routes.file(file.id);
	window.open(url, "_blank");
};

// Show context menu on image long tap
onMount(() => longtap(ref, () => menuState.update(() => true), 500));
</script>

<div class="gallery" bind:this={ref}>
	<div class="slides">
		{#each files as file, idx}
			<div>
				<img src={routes.file(file.small_id)} alt="See how good is this tree." />
				<div class="imgno">{idx + 1}/{files.length}</div>
				{#if file.added_at}
					<div class="date">{added_at(file)}</div>
				{/if}
				<button class="expand" onclick={() => onExpand(file)}>
					<ExpandIcon />
				</button>
			</div>
		{:else}
			<div>
				<img src="/tree.jpg" alt="no photos of this tree" />
			</div>
		{/each}
	</div>
</div>

<style>
	.gallery {
		position: relative;
		box-sizing: border-box;
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
