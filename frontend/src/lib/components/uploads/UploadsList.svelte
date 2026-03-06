<script lang="ts">
	import { locale } from '$lib/locale';
	import { loadUploads } from './UploadsList';
	import type { IUpload } from '$lib/db';
	import { Buttons, Button } from '$lib/ui';
	import { processUploadQueue, restartUploadQueue } from '$lib/upload';
	import { uploadStore } from '$lib/stores/upload';
	import UploadRow from './UploadRow.svelte';

	let uploads = $state<IUpload[]>([]);

	$effect(() => {
		return loadUploads((data) => {
			uploads = data;
		});
	});
</script>

<p class="hint">
	{#if $uploadStore.autoupload}
		{locale.uploadsEnabledHint()}
	{:else}
		{locale.uploadsDisabledHint()}
	{/if}
</p>

{#if uploads.length > 0}
	<Buttons>
		<Button type="button" onClick={processUploadQueue}>
			{locale.uploadsStart()}
		</Button>
		<Button type="secondary" onClick={restartUploadQueue}>
			{locale.uploadsRestart()}
		</Button>
	</Buttons>

	<div class="uploads-grid">
		{#each uploads as upload (upload.id)}
			<UploadRow {upload} />
		{/each}
	</div>
{:else}
	<p>{locale.uploadsEmpty()}</p>
{/if}

<style>
	.uploads-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 10px;
		margin-top: 1rem;
	}

	.hint {
		font-size: 0.8rem;
		opacity: 0.8;
		margin-bottom: 1rem;
		line-height: 1.4;
	}

	@media (min-width: 600px) {
		.uploads-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>
