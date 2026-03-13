<script lang="ts">
	import { locale } from '$lib/locale';
	import { NarrowPage, Form } from '$lib/ui';
	import PhotoUploader from './PhotoUploader.svelte';
	import UploadForm from './UploadForm.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { goto, routes } from '$lib/routes';
	import { load } from './hooks';

	const { data } = $props();
	const tree = data.tree;
	const treeId = data.id;

	const { handleChange } = load();

	const close = () => {
		goto(routes.mapPreview(data.id));
	};
</script>

<NarrowPage title={locale.photoTitle()} back={routes.mapPreview(data.id)}>
	<TreeForm {tree} onSubmit={close} onCancel={close}>
		<p>{locale.photoIntro()}</p>

		<PhotoUploader {treeId} onChange={handleChange} />

		<UploadForm id={tree.id} />
	</TreeForm>
</NarrowPage>
