<script>
import TabRenderer from "$lib/components/TabRenderer.svelte";
import { locale } from "$lib/locale";
import { routes } from "$lib/routes";

const { active } = $props();

const tabs = [
	{
		id: "new",
		title: locale.updatesAdded(),
		url: routes.newTrees(),
	},
	{
		id: "changes",
		title: locale.updatesChanged(),
		url: routes.changedTrees(),
	},
	{
		id: "comments",
		title: locale.updatesComments(),
		url: routes.comments(),
	},
];
</script>

<TabRenderer {active} {tabs} />
