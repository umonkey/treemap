<script>
	import { routes } from '$lib/routes';
	import TabRenderer from '$lib/components/TabRenderer.svelte';

	const { active } = $props();

	const tabs = [
		{
			id: 'new',
			title: 'Added',
			url: routes.newTrees()
		},
		{
			id: 'changes',
			title: 'Edited',
			url: routes.changedTrees()
		},
		{
			id: 'comments',
			title: 'Comments',
			url: routes.comments()
		}
	];
</script>

<TabRenderer {active} {tabs} />
