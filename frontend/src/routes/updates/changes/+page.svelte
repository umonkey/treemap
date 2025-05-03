<script lang="ts">
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import Tabs from '$lib/components/updates/Tabs.svelte';
	import { NarrowPage } from '$lib/ui';

	const { data } = $props();
	const { trees } = data;
</script>

<NarrowPage title="Changed Trees">
	<Tabs active="changes" />

	<div class="trees">
		{#each trees as tree}
			<NewTreesListItem {tree} />
		{/each}
	</div>
</NarrowPage>

<style>
	.trees {
		margin: var(--gap) 0;
	}
</style>
