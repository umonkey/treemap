<script lang="ts">
	import { NarrowPage } from '$lib/ui';
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import Tabs from '$lib/components/updates/Tabs.svelte';
	import { locale } from '$lib/locale';

	const { data } = $props();
	const { trees } = data;
</script>

<NarrowPage title={locale.updatesNewTitle()} nopadding>
	<Tabs active="new" />

	<div class="trees padded">
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
