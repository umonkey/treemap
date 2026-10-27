<script lang="ts">
	import { page } from '$app/state';
	import TabRenderer from '$lib/components/TabRenderer.svelte';
	import type { ISingleTree } from '$lib/types';
	import { activeTreeTab, treeTabs } from './TreeTabs.svelte.ts';

	const { tree }: { tree: ISingleTree } = $props();

	// URL-derived, so tab links follow the URL's id rather than a payload that
	// may still be the previous tree while a new one is loading.
	const id = $derived(page.params.id as string);

	const photos = $derived(tree.files?.length ?? 0);
	const comments = $derived(tree.comment_count ?? 0);

	const tabs = $derived(
		treeTabs(id).map((tab) => ({
			...tab,
			badge:
				tab.id === 'photos'
					? photos > 0
						? photos
						: undefined
					: tab.id === 'comments' && comments > 0
						? comments
						: undefined
		}))
	);

	const active = $derived(activeTreeTab(page.url.pathname, id)?.id);
</script>

<TabRenderer {active} {tabs} />
