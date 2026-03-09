<script lang="ts">
	import TabRenderer from '$lib/components/TabRenderer.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	const { active } = $props<{
		active: string;
	}>();

	const tabs = [
		{
			id: 'trees',
			title: locale.tabTrees(),
			url: routes.treeUpdates()
		},
		{
			id: 'comments',
			title: locale.updatesComments(),
			url: routes.comments()
		}
	];
</script>

<TabRenderer {active} {tabs} />
