<script lang="ts">
	import { page } from '$app/state';
	import TabRenderer from '$lib/components/TabRenderer.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import type { ISingleTree } from '$lib/types';

	const { tree }: { tree: ISingleTree } = $props();

	const photos = $derived(tree.files?.length ?? 0);
	const comments = $derived(tree.comment_count ?? 0);

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
			id: 'comments',
			title: locale.treeTabsComments(),
			url: routes.treeComments(tree.id),
			badge: comments > 0 ? comments : undefined
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

	const active = $derived(tabs.find((tab) => tab.url === page.url.pathname)?.id);
</script>

<TabRenderer {active} {tabs} />
