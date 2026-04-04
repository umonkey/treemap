<script lang="ts">
	import { DotsIcon } from '$lib/icons';
	import { routes } from '$lib/routes';
	import { getUser } from '$lib/stores/userStore';
	import type { ITreeFile } from '$lib/types';
	import { formatDate } from '$lib/utils/strings';
	import PhotoContextMenu from './PhotoContextMenu.svelte';
	import { pageState } from './page.svelte';

	let selectedFile = $state<ITreeFile | null>(null);
	let menuVisible = $state(false);

	const openMenu = (file: ITreeFile) => {
		selectedFile = file;
		menuVisible = true;
	};

	const closeMenu = () => {
		menuVisible = false;
		selectedFile = null;
	};

	const onMakeThumbnail = () => {
		if (selectedFile) {
			pageState.handleMakeThumbnail(selectedFile);
		}
		closeMenu();
	};

	const onDelete = () => {
		if (selectedFile) {
			pageState.handleDelete(selectedFile.id);
		}
		closeMenu();
	};
</script>

{#if pageState.loading}
	<!-- loading -->
{:else if pageState.error}
	<p>{pageState.error}</p>
{:else if pageState.tree.files.length > 0}
	<h2>Manage existing photos</h2>

	<div class="pics">
		{#each pageState.tree.files as file}
			{@const user = file.added_by ? $getUser(file.added_by) : null}
			<div class="pic">
				<a href={routes.file(file.source_id ?? file.large_id)} class="thumbnail" target="_blank">
					<img src={routes.file(file.small_id)} alt="thumbnail" />
				</a>

				<div class="overlay">
					{file.added_at ? formatDate(file.added_at) : ''} — {user?.name ?? 'unknown'}
				</div>

				<button class="menu-trigger" onclick={() => openMenu(file)} type="button">
					<DotsIcon />
				</button>
			</div>
		{/each}
	</div>

	<PhotoContextMenu visible={menuVisible} onClose={closeMenu} {onMakeThumbnail} {onDelete} />
{/if}

<style>
	.pics {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--gap);
		width: 100%;
	}

	@media (min-width: 600px) {
		.pics {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.pic {
		position: relative;
		aspect-ratio: 1 / 1;
		overflow: hidden;
		border-radius: 4px;
	}

	.thumbnail {
		display: block;
		width: 100%;
		height: 100%;

		img {
			width: 100%;
			height: 100%;
			object-position: center;
			object-fit: cover;
		}
	}

	.overlay {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		background: rgba(0, 0, 0, 0.7);
		color: #fff;
		font-size: 0.8rem;
		padding: 4px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		pointer-events: none;
	}

	.menu-trigger {
		position: absolute;
		top: 0;
		right: 0;
		width: 30px;
		height: 30px;
		background: rgba(0, 0, 0, 0.7);
		border: none;
		border-radius: 0 0 0 8px;
		color: #fff;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;

		:global(svg) {
			width: 16px;
			height: 16px;
		}

		&:hover {
			background: rgba(0, 0, 0, 0.6);
		}
	}
</style>
