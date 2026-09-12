<script lang="ts">
	import { pageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import { formatDateTime } from '$lib/utils/strings';
	import { hasPermission } from '$lib/stores/authStore';
	import PanoramaViewer from '$lib/components/panoramas/PanoramaViewer.svelte';

	const id = $derived(page.params.id as string);

	const capturedAt = $derived(
		pageState.image?.captured_at ? formatDateTime(pageState.image.captured_at) : ''
	);

	const canEdit = $derived($hasPermission('pano:edit'));

	$effect(() => {
		pageState.reload(id);
	});

	$effect(() => {
		return pageState.cleanup;
	});
</script>

<svelte:head>
	<title>360 Panorama</title>
</svelte:head>

<div class="preview">
	<div class="content">
		{#if pageState.image}
			<PanoramaViewer
				image={pageState.image}
				angle={pageState.angle}
				{canEdit}
				onMove={pageState.handleMove}
				onClose={pageState.handleClose}
			/>
			{#if capturedAt}
				<div class="control timestamp">
					{capturedAt}
				</div>
			{/if}
		{/if}
	</div>
</div>

<style>
	.preview {
		z-index: 2;
		display: flex;
		flex-direction: column;
		gap: var(--gap);
		padding: 0;
		background-color: var(--map-menu-background);
		box-sizing: border-box;
		position: relative;
	}

	.timestamp {
		position: absolute;
		bottom: 0;
		left: 0;
		width: auto;
		height: auto;
		padding: 4px 8px;
		font-size: 12px;
		z-index: 1;
		cursor: default;
		background-color: rgba(0, 0, 0, 0.75);
		color: white;
		border-radius: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;

		&:hover {
			background-color: #000;
		}
	}

	.content {
		flex-grow: 1;
		min-height: 0;
		display: flex;
		flex-direction: column;
		position: relative;
	}

	/* Mobile styles */
	@media screen and (max-width: 1023px) {
		.preview {
			position: fixed;
			bottom: var(--bottom-nav-height);
			left: 0;
			right: 0;
			height: 300px;
			border-top-left-radius: 8px;
			border-top-right-radius: 8px;
			animation: slideUp 0.2s ease-out;
		}
	}

	/* Desktop styles */
	@media screen and (min-width: 1024px) {
		.preview {
			position: fixed;
			bottom: var(--gap);
			left: var(--gap);
			width: 400px;
			height: 300px;
			border: none;
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
