<script lang="ts">
	import { routes } from '$lib/routes';
	import { FEATURES } from '$lib/features';

	import ChatIcon from '$lib/icons/ChatIcon.svelte';
	import HeartIcon from '$lib/icons/HeartIcon.svelte';
	import SaveIcon from '$lib/icons/SaveIcon.svelte';
	import CameraIcon from '$lib/icons/CameraIcon.svelte';
	import ShareButton from '$lib/components/tree/ShareButton.svelte';

	const { tree } = $props();
</script>

<div class="actions">
	{#if FEATURES.likes}
		<div><HeartIcon /></div>
	{/if}
	<div><a href={routes.treeComments(tree)}><ChatIcon /></a></div>
	<div><a href={routes.treeUploadPhotos(tree)}><CameraIcon /></a></div>
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
		}

		.sep {
			flex-grow: 1;
		}

		a {
			color: inherit;
		}
	}
</style>
