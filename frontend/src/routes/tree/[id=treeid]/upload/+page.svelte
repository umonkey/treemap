<script lang="ts">
	import { locale } from '$lib/locale';
	import { Button, Buttons, NarrowPage, Form, AuthWrapper } from '$lib/ui';
	import PhotoUploader from '$lib/components/PhotoUploader.svelte';
	import { UploadForm } from '$lib/forms';
	import { load } from './hooks';

	const { data } = $props();
	const tree = data.tree;
	const treeId = data.id;

	const { canSubmit, hasFiles, handleChange, handleSubmit } = load(treeId);
</script>

<NarrowPage title={locale.photoTitle()}>
	<AuthWrapper>
		<Form>
			<p>{locale.photoIntro()}</p>

			<PhotoUploader {treeId} onChange={handleChange} />

			{#if $hasFiles}
				<Buttons>
					<Button type="submit" onClick={handleSubmit} disabled={!$canSubmit}
						>{locale.photoUpload()}</Button
					>
				</Buttons>
			{/if}
		</Form>

		<UploadForm id={tree.id} />
	</AuthWrapper>
</NarrowPage>
