<script lang="ts">
	import { page } from '$app/stores';
	import AutoUploadCheckbox from '$lib/components/AutoUploadCheckbox.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { locale } from '$lib/locale';
	import { goto, routes } from '$lib/routes';
	import PhotoUploader from './PhotoUploader.svelte';
	import UploadForm from './UploadForm.svelte';
	import { pageState } from './page.svelte';

	const id = $derived($page.params.id as string);

	const close = () => {
		goto(routes.mapPreview(pageState.treeId));
	};

	$effect(() => {
		pageState.reload(id);
	});
</script>

<svelte:head>
	<title>{locale.photoTitle()} — {locale.appTitle()}</title>
</svelte:head>

<TreeForm tree={pageState.tree} title={locale.photoTitle()} onSubmit={close} onCancel={close}>
	<p>{locale.photoIntro()}</p>
	<PhotoUploader treeId={pageState.treeId} onChange={pageState.handleChange} />
	<AutoUploadCheckbox />
	<UploadForm />
</TreeForm>
