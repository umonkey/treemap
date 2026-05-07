<script lang="ts">
	import TabList from '$lib/ui/tab-list/TabList.svelte';
	import { componentState } from './Tabs.svelte.ts';
</script>

<TabList items={componentState.tabs} />
