<script lang="ts">
	import { AuthWrapper, Buttons, Button, Form, TextInput, FileUploader } from '$lib/ui';
	import { locale } from '$lib/locale';
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
	{:else}
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
