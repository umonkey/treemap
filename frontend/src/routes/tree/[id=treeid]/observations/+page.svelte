<script lang="ts">
	import ObservationForm from '$lib/components/observation/ObservationForm.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { NarrowPage } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { formatSpecies } from '$lib/utils/trees';
	import Title from '$lib/components/tree/Title.svelte';
	import { routes } from '$lib/routes';

	const { data } = $props();
	const tree = data.tree;
</script>

<NarrowPage title={locale.treeTabsObservations()} back={routes.mapPreview(data.id)} nopadding>
	<Title title={formatSpecies(tree.species)} address={tree.address} />
	<TreeContextMenu id={tree.id} />

	<div class="padded">
		<ObservationForm id={tree.id} />
	</div>
</NarrowPage>

<style>
	.padded {
		padding: 0 var(--gap);
	}
</style>
