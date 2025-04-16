<script lang="ts">
import Header from "$lib/components/tree/Header.svelte";
import NewTreesListItem from "$lib/components/updates/NewTreesListItem.svelte";
import Tabs from "$lib/components/updates/Tabs.svelte";
import { locale } from "$lib/locale";

const { data } = $props();
const { trees } = data;
</script>

<svelte:head>
	<title>{locale.updatesNewTitle()}</title>
</svelte:head>

<Header title={locale.updatesNewTitle()} />
<Tabs active="new" />

<div class="trees padded">
	{#each trees as tree}
		<NewTreesListItem {tree} />
	{/each}
</div>

<style>
	.trees {
		margin: var(--gap) 0;
	}
</style>
