<script lang="ts">
	import { Header, NarrowPage, TabList } from '$lib/ui';
	import UploadsList from '$lib/components/uploads/UploadsList.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	const tabs = [
		{ title: locale.profileTitle(), link: routes.profile() },
		{ title: locale.uploadsTitle(), link: routes.uploads(), active: true }
	];
</script>

<svelte:head>
	<title>{locale.uploadsTitle()}</title>
</svelte:head>

<Header title={locale.uploadsTitle()} />

<TabList items={tabs} />

<NarrowPage>
	<UploadsList />
</NarrowPage>
