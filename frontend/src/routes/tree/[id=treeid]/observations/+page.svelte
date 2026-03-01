<script lang="ts">
	import { ObservationForm } from '$lib/forms';
	import { NarrowPage, AuthWrapper, TreeTabs, TreeContextMenu } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { formatSpecies } from '$lib/utils/trees';
	import Title from '$lib/components/tree/Title.svelte';

	const { data } = $props();
	const tree = data.tree;
</script>

<NarrowPage title={locale.treeTabsObservations()} nopadding>
	<Title title={formatSpecies(tree.species)} address={tree.address} />
	<TreeTabs tree={tree.id} active="observations" comment_count={tree.comment_count} />
	<TreeContextMenu id={tree.id} />

	<div class="padded">
		<AuthWrapper>
			<ObservationForm id={tree.id} />
		</AuthWrapper>
	</div>
</NarrowPage>

<style>
	.padded {
		padding: 0 var(--gap);
	}
</style>
