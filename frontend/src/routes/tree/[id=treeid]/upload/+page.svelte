<script lang="ts">
	import { locale } from '$lib/locale';
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import { Button, Buttons, NarrowPage, Form, FileUploader } from '$lib/ui';
	import { UploadForm } from '$lib/forms';
	import { load } from './hooks';

	const { data } = $props();
	const tree = data.tree;
	const treeId = data.id;

	const { canSubmit, hasFiles, handleBusy, handleChange, handleSubmit } = load(treeId);
</script>

<NarrowPage title={locale.photoTitle()}>
	<AuthWrapper>
		<Form>
			<p>{locale.photoIntro()}</p>

			<FileUploader onBusy={handleBusy} onChange={handleChange} />

			{#if $hasFiles}
				<Buttons>
					<Button
						label={locale.photoUpload()}
						type="submit"
						onClick={handleSubmit}
						disabled={!$canSubmit}
					/>
				</Buttons>
			{/if}
		</Form>

		<UploadForm id={tree.id} />
	</AuthWrapper>
</NarrowPage>
