<script lang="ts">
	import TabRenderer from '$lib/components/TabRenderer.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import type { ISingleTree } from '$lib/types';

	const { tree, active }: { tree: ISingleTree; active: string } = $props();

	const photos = $derived(tree.files?.length ?? 0);

	const tabs = $derived([
		{
			id: 'preview',
			title: locale.treeTabsPreview(),
			url: routes.mapPreview(tree.id)
		},
		{
			id: 'photos',
			title: locale.treeTabsPhotos(),
			url: routes.treePhotos(tree.id),
			badge: photos > 0 ? photos : undefined
		},
		{
			id: 'observations',
			title: locale.treeTabsObservations(),
			url: routes.treeObservations(tree.id)
		},
		{
			id: 'history',
			title: locale.treeTabsChanges(),
			url: routes.treeHistory(tree.id)
		}
	]);
</script>

<TabRenderer {active} {tabs} />
