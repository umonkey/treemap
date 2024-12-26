<script>
	import { routes } from '$lib/routes';
	import { locale } from '$lib/locale';
	import TabRenderer from '$lib/components/TabRenderer.svelte';

	const { active } = $props();

	const tabs = [
		{
			id: 'new',
			title: locale.updatesAdded(),
			url: routes.newTrees()
		},
		{
			id: 'changes',
			title: locale.updatesChanged(),
			url: routes.changedTrees()
		},
		{
			id: 'comments',
			title: locale.updatesComments(),
			url: routes.comments()
		}
	];
</script>

<TabRenderer {active} {tabs} />
