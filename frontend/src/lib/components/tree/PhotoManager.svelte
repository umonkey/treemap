<script lang="ts">
	import { apiClient } from '$lib/api';
	import { routes } from '$lib/routes';
	import { fileAttribution } from '$lib/utils/strings';
	import { toast } from '@zerodevx/svelte-toast';
	import type { ITreeFile } from '$lib/types';

	const { tree } = $props();

	let thumbnail_id = $state(tree.thumbnail_id);

	const handleMakeThumbnail = async (file: ITreeFile) => {
		const res = await apiClient.changeTreeThumbnail(tree.id, file.id);

		if (res.status >= 200 && res.status < 300) {
			thumbnail_id = file.small_id;
			toast.push('Thumbnail changed.');
		} else {
			toast.push('Error changing thumbnail.');
		}
	};

	const handleDelete = async (id: string) => {
		const res = await apiClient.deleteFile(id);

		if (res.status >= 200 && res.status < 300) {
			toast.push('File deleted.');
		} else {
			toast.push('Error deleting file.');
		}
	};
</script>

{#if tree.files.length > 0}
	<h2>Manage existing photos</h2>

	<div class="pics">
		{#each tree.files as file}
			<div class="pic">
				<a href={routes.file(file.id)} class="thumbnail" target="_blank">
					<img src={routes.file(file.small_id)} alt="thumbnail" />
				</a>
				<div class="props">
					<div class="by">{fileAttribution(file)}</div>
					<div class="actions">
						<button
							class="button"
							type="button"
							disabled={file.small_id === thumbnail_id}
							onclick={() => handleMakeThumbnail(file)}>Make thumbnail</button
						>
						<button
							class="button"
							type="button"
							disabled={file.small_id === thumbnail_id}
							onclick={() => handleDelete(file.id)}>Delete</button
						>
					</div>
				</div>
			</div>
		{/each}
	</div>
{/if}

<style>
	.pics {
		display: flex;
		flex-direction: column;
		gap: var(--gap);
		width: 100%;
	}

	.pic {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
	}

	.thumbnail {
		display: block;
		position: relative;
		line-height: 0;
		flex-basis: 100px;
		flex-grow: 0;
		flex-shrink: 0;

		img {
			width: 100%;
			aspect-ratio: 1;
			object-position: center;
			object-fit: cover;
		}
	}

	.props {
		display: flex;
		flex-direction: column;
		gap: var(--gap);
	}
</style>
