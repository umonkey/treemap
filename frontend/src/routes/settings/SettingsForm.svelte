<script lang="ts">
	import { locale } from '$lib/locale';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import FileUploader from '$lib/ui/file-uploader/FileUploader.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import { hooks } from './hooks';

	const {
		loading,
		saving,
		error,
		saveError,
		data,
		handleSave,
		handleCancel,
		handleFileBusy,
		handleFileChange,
		handleNameChange
	} = hooks();
</script>

<AuthWrapper>
	{#if $loading}
		<!-- loading -->
	{:else if $error}
		<p>{$error}</p>
	{:else if $data}
		<Form onSubmit={handleSave}>
			<TextInput label="Display name:" value={$data.name} onChange={handleNameChange} />

			<FileUploader
				label="Update profile picture:"
				onBusy={handleFileBusy}
				onChange={handleFileChange}
				single
			/>

			<Buttons>
				<Button type="submit" onClick={handleSave} disabled={$saving}>{locale.editSave()}</Button>
				<Button type="cancel" onClick={handleCancel}>{locale.editCancel()}</Button>
			</Buttons>
		</Form>

		{#if $saveError}
			<p>{$saveError}</p>
		{/if}
	{/if}
</AuthWrapper>
