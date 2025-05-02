<script lang="ts">
	import { locale } from '$lib/locale';
	import { fileStore, isUploading, storedFiles, uploadMessage } from '$lib/stores/fileStore';
	import { startUpload } from '$lib/utils/fileUploader';
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import { FilePicker, Header } from '$lib/ui';
	import { UploadForm } from '$lib/forms';
	import { CloseIcon } from '$lib/icons';

	const { data } = $props();
	const tree = data.tree;
	const treeId = data.id;

	const onFileSelected = (selected: File[]) => {
		fileStore.update((store) => {
			store.files = [...store.files, ...selected];
			return store;
		});
	};

	const removeFile = (index: number) => {
		console.debug(`Removing file at index ${index}.`);

		fileStore.update((store) => {
			const newFiles = [...store.files];
			newFiles.splice(index, 1);
			store.files = newFiles;
			return store;
		});
	};
</script>

<svelte:head>
	<title>{locale.photoTitle()}</title>
</svelte:head>

<Header title={locale.photoTitle()} />

<div class="padded">
	<AuthWrapper>
		<div class="form">
			<p>{locale.photoIntro()}</p>

			{#if $uploadMessage}
				<p>{$uploadMessage}</p>
			{/if}

			<div class="buttons">
				<FilePicker {onFileSelected} disabled={$isUploading} />
				<button
					disabled={!($storedFiles.length > 0 && !$isUploading)}
					class="button"
					type="button"
					onclick={() => startUpload(treeId)}>{locale.photoUpload()}</button
				>
			</div>

			{#if $storedFiles.length > 0}
				<h2>Selected photos:</h2>

				<div class="grid">
					{#each $storedFiles as file, idx}
						<div class="preview">
							<img src={URL.createObjectURL(file)} alt="preview" />
							<button class="close" onclick={() => removeFile(idx)}>
								<CloseIcon width="24px" height="24px" />
							</button>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		<UploadForm id={tree.id} />
	</AuthWrapper>
</div>

<style>
	.grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		grid-gap: var(--gap);

		margin: var(--gap) 0;

		.preview {
			line-height: 0;
			position: relative;
		}

		img {
			display: block;
			width: 100%;
			aspect-ratio: 1/1;
			object-position: center;
			object-fit: contain;
			border: 1px solid var(--sep-color);
		}

		button.close {
			width: 24px;
			height: 24px;
			border: none;
			background-color: transparent;
			color: var(--text-color);
			top: 10px;
			right: 10px;
			position: absolute;
			cursor: pointer;
		}
	}
</style>
