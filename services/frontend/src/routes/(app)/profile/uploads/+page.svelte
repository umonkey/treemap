<script lang="ts">
	import AutoUploadCheckbox from '$lib/components/AutoUploadCheckbox.svelte';
	import { uploadStore } from '$lib/stores/upload';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import { onMount } from 'svelte';
	import UploadRow from './UploadRow.svelte';
	import UploadContextMenu from './UploadContextMenu.svelte';
	import { locale } from './lang';
	import { pageState } from './page.svelte';
	import type { IUpload } from '$lib/db';

	let selectedUpload = $state<IUpload | null>(null);
	let menuVisible = $state(false);

	onMount(pageState.onMount);

	const openMenu = (upload: IUpload) => {
		selectedUpload = upload;
		menuVisible = true;
	};

	const closeMenu = () => {
		menuVisible = false;
		selectedUpload = null;
	};

	const onDelete = () => {
		if (selectedUpload?.id !== undefined) {
			pageState.deleteUpload(selectedUpload.id);
		}
		closeMenu();
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

	<UploadContextMenu visible={menuVisible} onClose={closeMenu} {onDelete} />
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
