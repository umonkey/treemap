<script lang="ts">
	import ObservationForm from './ObservationForm.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { NarrowPage } from '$lib/ui';
	import { locale } from '$lib/locale';
	import Title from '$lib/components/tree/Title.svelte';
	import { routes } from '$lib/routes';

	const { data } = $props();
	const tree = data.tree;
</script>

<NarrowPage title={locale.treeTabsObservations()} back={routes.mapPreview(data.id)}>
	<Title title={tree.species} address={tree.address} />
	<TreeContextMenu id={tree.id} />

	<ObservationForm id={tree.id} />
</NarrowPage>
