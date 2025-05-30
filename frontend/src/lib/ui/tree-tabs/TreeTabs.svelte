<script lang="ts">
	import TabRenderer from '$lib/components/TabRenderer.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	const { tree, active, comment_count } = $props<{
		tree: string;
		active: string;
		comment_count?: number;
	}>();

	const tabs = [
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
			id: 'comments',
			title: locale.treeTabsComments(comment_count ?? 0),
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
