<script lang="ts">
	import TabRenderer from '$lib/components/TabRenderer.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	export let tree;
	export let active;

	const tabs = [
		{
			id: 'details',
			title: locale.treeTabsDetails(),
			url: routes.treeDetails(tree)
		},
		{
			id: 'map',
			title: locale.treeTabsMap(),
			url: routes.treeMap(tree)
		},
		{
			id: 'comments',
			title: locale.treeTabsComments(),
			url: routes.treeComments(tree)
		},
		{
			id: 'history',
			title: locale.treeTabsChanges(),
			url: routes.treeHistory(tree)
		}
	];
</script>

<TabRenderer {active} {tabs} />
