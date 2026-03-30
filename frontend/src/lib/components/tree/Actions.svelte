<script lang="ts">
	import ShareButton from '$lib/components/tree/ShareButton.svelte';
	import { FEATURES } from '$lib/features';
	import { CameraIcon, HeartIcon, HeartSolidIcon, SaveIcon } from '$lib/icons';
	import WikiIcon from '$lib/icons/WikipediaIcon.svelte';
	import OpenStreetMapIcon from '$lib/icons/OpenStreetMapIcon.svelte';
	import { like, preloadMeLikes, unlike } from '$lib/likes';
	import { routes } from '$lib/routes';
	import { isLiked } from '$lib/stores/likeStore';
	import type { ITree } from '$lib/types';
	import { onMount } from 'svelte';
	import { hooks } from './Actions';

	const { tree } = $props<{
		tree: ITree;
	}>();

	const isTreeLiked = $derived($isLiked(tree.id));
	const { actors, reload } = hooks();

	onMount(() => {
		preloadMeLikes();
		reload(tree.id);
	});

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
	<div class="icon">
		<a href="/" class:liked={isTreeLiked} onclick={onLike}
			>{#if isTreeLiked}<HeartSolidIcon />{:else}<HeartIcon />{/if}</a
		>
	</div>

	<div class="icon"><a href={routes.treeUploadPhotos(tree.id)}><CameraIcon /></a></div>

	{#if tree.species}
		<div class="icon">
			<a href="https://en.wikipedia.org/w/index.php?search={tree.species}" target="_blank">
				<WikiIcon />
			</a>
		</div>
	{/if}

	{#if tree.osm_id}
		<div class="icon">
			<a href={`https://www.openstreetmap.org/node/${tree.osm_id}`} target="_blank">
				<OpenStreetMapIcon />
			</a>
		</div>
	{/if}

	<div class="icon"><ShareButton id={tree.id} /></div>

	{#if FEATURES.bookmarks}
		<div class="icon"><SaveIcon /></div>
	{/if}

	<div class="sep"></div>

	{#if $actors.length > 0}
		<a class="actors" href={routes.treeHistory(tree.id)}>
			{#each $actors as user}
				<img src={user.picture} alt={user.name} referrerpolicy="no-referrer" />
			{/each}
		</a>
	{/if}
</div>

<style>
	.actions {
		display: flex;
		align-items: center;

		gap: calc(var(--gap) * 1.5);
		justify-content: left;
		margin: var(--gap) 0;
		padding: 0 var(--gap);

		.icon {
			width: 20px;
			height: 20px;
		}

		& > div {
			width: 30px;
			height: 30px;
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

	.actors {
		flex: 1 1 0;
		display: flex;
		justify-content: flex-end;
		height: 30px;

		img {
			width: 30px;
			height: 30px;
			object-fit: cover;
			object-position: center;
			border-radius: 50%;
			border: 2px solid #fff;
			margin-left: -15px;
			box-sizing: border-box;
		}
	}
</style>
