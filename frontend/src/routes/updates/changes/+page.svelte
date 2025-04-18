<script lang="ts">
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import Tabs from '$lib/components/updates/Tabs.svelte';
	import { Header } from '$lib/ui';

	const { data } = $props();
	const { trees } = data;
</script>

<svelte:head>
	<title>Changed Trees</title>
</svelte:head>

<Header title="Changed Trees" />
<Tabs active="changes" />

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
