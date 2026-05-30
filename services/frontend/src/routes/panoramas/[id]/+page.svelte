<script lang="ts">
	import { pageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import PanoramaViewer from './PanoramaViewer.svelte';

	const id = $derived(page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});
</script>

<div class="preview">
	<div class="header">
		<div class="title">
			Panorama {pageState.id}
		</div>
		<button class="close" onclick={pageState.handleClose} aria-label="Close">
			<CloseIcon />
		</button>
	</div>

	<div class="content">
		{#if pageState.image}
			<PanoramaViewer image={pageState.image} />
		{:else}
			<p>Image ID: {pageState.id}</p>
		{/if}
	</div>
</div>

<style>
	.preview {
		z-index: 2;
		display: flex;
		flex-direction: column;
		gap: var(--gap);
		padding: var(--gap);
		background-color: var(--map-menu-background);
		box-sizing: border-box;
	}

	.header {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;

		.close {
			width: 30px;
			height: 30px;
			cursor: pointer;
			background-color: transparent;
			border: none;
			color: light-dark(black, white);
			opacity: 0.5;
			display: flex;
			align-items: center;
			justify-content: center;

			&:hover {
				opacity: 1;
			}
		}
	}

	.title {
		flex-grow: 1;
		font-size: 120%;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.content {
		flex-grow: 1;
		min-height: 0;
	}

	/* Mobile styles */
	@media screen and (max-width: 1023px) {
		.preview {
			position: fixed;
			bottom: var(--bottom-nav-height);
			left: 0;
			right: 0;
			border-top-left-radius: 8px;
			border-top-right-radius: 8px;
			animation: slideUp 0.2s ease-out;
		}
	}

	/* Desktop styles */
	@media screen and (min-width: 1024px) {
		.preview {
			position: fixed;
			top: 0;
			left: 0;
			width: 300px;
			height: 100vh;
			border-right: 1px solid var(--color-dialog-border);
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
