<script lang="ts">
	import AutoUploadCheckbox from '$lib/components/AutoUploadCheckbox.svelte';
	import { menuBus } from '$lib/buses/menuBus';
	import type { IUpload } from '$lib/db';
	import { uploadStore } from '$lib/stores/upload';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import { onMount } from 'svelte';
	import { locale } from './lang';
	import { PageState } from './page.svelte';
	import UploadRow from './UploadRow.svelte';

	const pageState = new PageState();

	onMount(pageState.onMount);

	const openMenu = (upload: IUpload, event: MouseEvent) => {
		if (upload.id === undefined) {
			return;
		}

		menuBus.emit('showUpload', {
			uploadId: upload.id,
			x: event.clientX,
			y: event.clientY
		});
	};
</script>

<AuthWrapper>
	<AutoUploadCheckbox />

	<p class="hint">
		{#if $uploadStore.autoupload}
			{locale.uploadsEnabledHint()}
		{:else}
			{locale.uploadsDisabledHint()}
		{/if}
	</p>

	{#if pageState.uploads.length > 0}
		<Buttons>
			<Button type="button" onClick={pageState.processQueue} nowrap>
				{locale.uploadsStart()}
			</Button>
			<Button type="secondary" onClick={pageState.restartQueue} nowrap>
				{locale.uploadsRestart()}
			</Button>
		</Buttons>

		<div class="uploads-grid">
			{#each pageState.uploads as upload (upload.id)}
				<UploadRow {upload} onMenu={openMenu} />
			{/each}
		</div>
	{:else}
		<p>{locale.uploadsEmpty()}</p>
	{/if}
</AuthWrapper>

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
