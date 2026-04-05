<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import Tabs from '$lib/components/profile/Tabs.svelte';
	import { locale } from '$lib/locale';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import FileUploader from '$lib/ui/file-uploader/FileUploader.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});
</script>

<Dialog title={locale.settingsTitle()}>
	<Tabs active="settings" />

	<AuthWrapper>
		{#if pageState.loading}
			<!-- loading -->
		{:else if pageState.error}
			<p>{pageState.error}</p>
		{:else if pageState.data}
			<Form onSubmit={pageState.handleSave}>
				<TextInput
					label="Display name:"
					value={pageState.name}
					onChange={pageState.handleNameChange}
				/>

				<FileUploader
					label="Update profile picture:"
					onBusy={pageState.handleFileBusy}
					onChange={pageState.handleFileChange}
					single
				/>

				<Buttons>
					<Button type="submit" onClick={pageState.handleSave} disabled={pageState.saving}
						>{locale.editSave()}</Button
					>
					<Button type="cancel" onClick={pageState.handleCancel}>{locale.editCancel()}</Button>
				</Buttons>
			</Form>

			{#if pageState.saveError}
				<p>{pageState.saveError}</p>
			{/if}
		{/if}
	</AuthWrapper>
</Dialog>
