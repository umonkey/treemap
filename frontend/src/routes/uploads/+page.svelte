<script lang="ts">
	import { Header, NarrowPage } from '$lib/ui';
	import UploadsList from '$lib/components/uploads/UploadsList.svelte';
	import { locale } from '$lib/locale';
</script>

<svelte:head>
	<title>{locale.uploadsTitle()}</title>
</svelte:head>

<Header title={locale.uploadsTitle()} />

<NarrowPage>
	<UploadsList />
</NarrowPage>
