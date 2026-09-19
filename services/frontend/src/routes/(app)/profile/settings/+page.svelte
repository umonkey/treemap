<script lang="ts">
	import { untrack } from 'svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import FileUploader from '$lib/ui/file-uploader/FileUploader.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import DisplayNameInput from './DisplayNameInput.svelte';
	import WakeLockInput from './WakeLockInput.svelte';
	import { SettingsPage } from './page.svelte.ts';

	const pageState = new SettingsPage();

	$effect(() => {
		untrack(() => pageState.reload());
	});
</script>

<AuthWrapper>
	{#if pageState.loading}
		<!-- loading -->
	{:else if pageState.error}
		<p>{pageState.error}</p>
	{:else if pageState.data}
		<Form>
			<DisplayNameInput bind:value={pageState.name} onSave={pageState.handleNameBlur} />

			<WakeLockInput />

			<FileUploader
				label="Update profile picture:"
				onBusy={pageState.handleFileBusy}
				onChange={pageState.handleFileChange}
				single
			/>
		</Form>
	{/if}
</AuthWrapper>
