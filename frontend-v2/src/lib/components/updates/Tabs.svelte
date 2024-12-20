<script>
	import { routes } from '$lib/routes';
	import TabRenderer from '$lib/components/TabRenderer.svelte';

	const { active } = $props();

	const tabs = [
		{
			id: 'new',
			title: 'New Trees',
			url: routes.newTrees()
		},
		{
			id: 'changes',
			title: 'Changes',
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
