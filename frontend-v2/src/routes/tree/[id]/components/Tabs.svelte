<script>
	import { routes } from '$lib/routes';
	import TabRenderer from '$lib/components/TabRenderer.svelte';

	export let tree;
	export let active;

	const tabs = [
		{
			id: 'details',
			title: 'Details',
			url: routes.treeDetails(tree)
		},
		{
			id: 'map',
			title: 'Map',
			url: routes.treeMap(tree)
		},
		{
			id: 'comments',
			title: 'Comments',
			url: routes.treeComments(tree)
		},
		{
			id: 'history',
			title: 'History',
			url: routes.treeHistory(tree)
		}
	];
</script>

<TabRenderer {active} {tabs} />
