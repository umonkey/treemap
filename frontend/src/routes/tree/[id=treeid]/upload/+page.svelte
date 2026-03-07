<script lang="ts">
	import { locale } from '$lib/locale';
	import { NarrowPage, Form, AuthWrapper } from '$lib/ui';
	import PhotoUploader from '$lib/components/photos/PhotoUploader.svelte';
	import UploadForm from '$lib/components/photos/UploadForm.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { routes } from '$lib/routes';
	import { load } from './hooks';

	const { data } = $props();
	const tree = data.tree;
	const treeId = data.id;

	const { handleChange } = load();
</script>

<NarrowPage title={locale.photoTitle()} back={routes.mapPreview(data.id)}>
	<AuthWrapper>
		<Title title={tree.species} address={tree.address} />
		<TreeContextMenu id={tree.id} />

		<Form>
			<p>{locale.photoIntro()}</p>

			<PhotoUploader {treeId} onChange={handleChange} />
		</Form>

		<UploadForm id={tree.id} />
	</AuthWrapper>
</NarrowPage>
