<script lang="ts">
	import TabList from '$lib/ui/tab-list/TabList.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	const { active } = $props<{
		active: string;
	}>();

	const tabs = [
		{
			id: 'trees',
			title: locale.tabTrees(),
			link: routes.treeUpdates(),
			active: active === 'trees'
		},
		{
			id: 'comments',
			title: locale.updatesComments(),
			link: routes.comments(),
			active: active === 'comments'
		}
	];
</script>

<TabList items={tabs} />
