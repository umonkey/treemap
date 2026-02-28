<script lang="ts">
	import { locale } from '$lib/locale';
	import { loadUploads } from './UploadsList';
	import type { IUpload } from '$lib/db';
	import { Buttons, Button } from '$lib/ui';
	import { processUploadQueue, restartUploadQueue } from '$lib/upload';
	import UploadRow from './UploadRow.svelte';

	let uploads = $state<IUpload[]>([]);

	$effect(() => {
		return loadUploads((data) => {
			uploads = data;
		});
	});
</script>

{#if uploads.length > 0}
	<div class="uploads-grid">
		{#each uploads as upload (upload.id)}
			<UploadRow {upload} />
		{/each}
	</div>

	<Buttons>
		<Button type="button" onClick={processUploadQueue}>
			{locale.uploadsStart()}
		</Button>
		<Button type="secondary" onClick={restartUploadQueue}>
			{locale.uploadsRestart()}
		</Button>
	</Buttons>
{:else}
	<p>{locale.uploadsEmpty()}</p>
{/if}

<style>
	.uploads-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 10px;
		margin-bottom: 1rem;
	}

	@media (min-width: 600px) {
		.uploads-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>
