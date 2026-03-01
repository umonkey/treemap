<script lang="ts">
	import { locale } from '$lib/locale';
	import { NarrowPage, Form, AuthWrapper } from '$lib/ui';
	import PhotoUploader from '$lib/components/PhotoUploader.svelte';
	import { UploadForm } from '$lib/forms';
	import { load } from './hooks';

	const { data } = $props();
	const tree = data.tree;
	const treeId = data.id;

	const { handleChange } = load();
</script>

<NarrowPage title={locale.photoTitle()}>
	<AuthWrapper>
		<Form>
			<p>{locale.photoIntro()}</p>

			<PhotoUploader {treeId} onChange={handleChange} />
		</Form>

		<UploadForm id={tree.id} />
	</AuthWrapper>
</NarrowPage>
