<script lang="ts">
	import TabRenderer from '$lib/components/TabRenderer.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	const { tree, active }: { tree: string; active: string } = $props();

	const tabs = $derived([
		{
			id: 'details',
			title: locale.treeTabsDetails(),
			url: routes.treeDetails(tree)
		},
		{
			id: '360',
			title: locale.treeTabsStreetView(),
			url: routes.treeStreetView(tree)
		},
		{
			id: 'history',
			title: locale.treeTabsChanges(),
			url: routes.treeHistory(tree)
		}
	]);
</script>

<TabRenderer {active} {tabs} />
