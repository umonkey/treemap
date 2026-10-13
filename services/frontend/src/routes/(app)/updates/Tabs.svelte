<script lang="ts">
	import TabList from '$lib/ui/tab-list/TabList.svelte';
	import { TabsLogic } from './Tabs.svelte.ts';

	const componentState = new TabsLogic();
</script>

<TabList items={componentState.tabs} />
