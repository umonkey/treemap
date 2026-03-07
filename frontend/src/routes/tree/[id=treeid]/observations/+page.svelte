<script lang="ts">
	import ObservationForm from '$lib/components/observation/ObservationForm.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import TreeTabs from '$lib/components/tree/TreeTabs.svelte';
	import { NarrowPage } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { formatSpecies } from '$lib/utils/trees';
	import Title from '$lib/components/tree/Title.svelte';

	const { data } = $props();
	const tree = data.tree;
</script>

<NarrowPage title={locale.treeTabsObservations()} nopadding>
	<Title title={formatSpecies(tree.species)} address={tree.address} />
	<TreeTabs tree={tree.id} active="observations" />
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
