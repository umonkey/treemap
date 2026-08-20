<script lang="ts">
	import { page } from '$app/state';
	import { locale } from './lang';
	import SearchResultsSidebar from './SearchResultsSidebar.svelte';

	const query = $derived(page.url.searchParams.get('query') ?? '');
</script>

<svelte:head>
	<title>{locale.tabTitle(query)}</title>
</svelte:head>

<SearchResultsSidebar {query} />
