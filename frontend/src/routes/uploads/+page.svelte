<script lang="ts">
	import AutoUploadCheckbox from '$lib/components/AutoUploadCheckbox.svelte';
	import UploadRow from '$lib/components/uploads/UploadRow.svelte';
	import { Buttons, Button } from '$lib/ui';
	import { Header, TabList } from '$lib/ui';
	import { locale } from './lang';
	import { onMount } from 'svelte';
	import { processUploadQueue, restartUploadQueue } from '$lib/upload';
	import { routes } from '$lib/routes';
	import { uploadStore } from '$lib/stores/upload';
	import { hooks } from './hooks';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const tabs = $derived([
		{ title: locale.profileTitle(), link: routes.profile() },
		{
			title: locale.uploadsTitle(),
			link: routes.uploads(),
			active: true,
			badge: $uploadStore.pending > 0 ? $uploadStore.pending : undefined
		}
	]);

	const { uploads } = hooks(onMount);
</script>

<Dialog title={locale.uploadsTitle()}>
	<AuthWrapper>
		<TabList items={tabs} />

		<AutoUploadCheckbox />

		<p class="hint">
			{#if $uploadStore.autoupload}
				{locale.uploadsEnabledHint()}
			{:else}
				{locale.uploadsDisabledHint()}
			{/if}
		</p>

		{#if $uploads.length > 0}
			<Buttons>
				<Button type="button" onClick={processUploadQueue} nowrap>
					{locale.uploadsStart()}
				</Button>
				<Button type="secondary" onClick={restartUploadQueue} nowrap>
					{locale.uploadsRestart()}
				</Button>
			</Buttons>

			<div class="uploads-grid">
				{#each $uploads as upload (upload.id)}
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
