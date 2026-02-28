<script lang="ts">
	import { Header, NarrowPage, TabList } from '$lib/ui';
	import UploadsList from '$lib/components/uploads/UploadsList.svelte';
	import AutoUploadCheckbox from '$lib/components/AutoUploadCheckbox.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { uploadStore } from '$lib/stores/upload';

	const tabs = $derived([
		{ title: locale.profileTitle(), link: routes.profile() },
		{
			title: locale.uploadsTitle(),
			link: routes.uploads(),
			active: true,
			badge: $uploadStore.pending > 0 ? $uploadStore.pending : undefined
		}
	]);
</script>

<svelte:head>
	<title>{locale.uploadsTitle()}</title>
</svelte:head>

<Header title={locale.uploadsTitle()} />

<TabList items={tabs} />

<NarrowPage>
	<AutoUploadCheckbox />
	<UploadsList />
</NarrowPage>
