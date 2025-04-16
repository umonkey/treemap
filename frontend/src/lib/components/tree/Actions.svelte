<script lang="ts">
import { FEATURES } from "$lib/features";
import { like, preloadMeLikes, unlike } from "$lib/likes";
import { routes } from "$lib/routes";
import { isLiked } from "$lib/stores/likeStore";
import type { ITree } from "$lib/types";
import { onMount } from "svelte";

import ShareButton from "$lib/components/tree/ShareButton.svelte";
import CameraIcon from "$lib/icons/CameraIcon.svelte";
import ChatIcon from "$lib/icons/ChatIcon.svelte";
import HeartIcon from "$lib/icons/HeartIcon.svelte";
import HeartSolidIcon from "$lib/icons/HeartSolidIcon.svelte";
import SaveIcon from "$lib/icons/SaveIcon.svelte";

const {
	tree,
}: {
	tree: ITree;
} = $props();

const isTreeLiked = $derived($isLiked(tree.id));

onMount(preloadMeLikes);

const onLike = async (e: Event) => {
	e.preventDefault();

	if (!isTreeLiked) {
		await like(tree.id);
	} else {
		await unlike(tree.id);
	}
};
</script>

<div class="actions">
	<div>
		<a href="/" class:liked={isTreeLiked} onclick={onLike}
			>{#if isTreeLiked}<HeartSolidIcon />{:else}<HeartIcon />{/if}</a
		>
	</div>
	<div><a href={routes.treeComments(tree.id)}><ChatIcon /></a></div>
	<div><a href={routes.treeUploadPhotos(tree.id)}><CameraIcon /></a></div>
	<div><ShareButton /></div>
	{#if FEATURES.bookmarks}
		<div class="sep"></div>
		<div><SaveIcon /></div>
	{/if}
</div>

<style>
	.actions {
		display: flex;
		gap: calc(var(--gap) * 1.5);
		justify-content: left;
		margin: var(--gap) 0;
		padding: 0 var(--gap);

		& > div {
			width: 20px;
			height: 20px;
			color: var(--icon-color-secondary);
			text-align: center;

			a {
				display: block;
				width: 100%;
				height: 100%;
			}
		}

		.sep {
			flex-grow: 1;
		}

		a {
			color: inherit;
		}

		.liked {
			color: #ff3040;
		}
	}
</style>
