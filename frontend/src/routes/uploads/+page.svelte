<script lang="ts">
	import AutoUploadCheckbox from '$lib/components/AutoUploadCheckbox.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import Tabs from '$lib/components/profile/Tabs.svelte';
	import { uploadStore } from '$lib/stores/upload';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import { onMount } from 'svelte';
	import UploadRow from './UploadRow.svelte';
	import { locale } from './lang';
	import { pageState } from './page.svelte';

	onMount(pageState.onMount);
</script>

<Dialog title={locale.uploadsTitle()}>
	<AuthWrapper>
		<Tabs active="uploads" />

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
					<UploadRow {upload} />
				{/each}
			</div>
		{:else}
			<p>{locale.uploadsEmpty()}</p>
		{/if}
	</AuthWrapper>
</Dialog>

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
