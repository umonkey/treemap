<script lang="ts">
	import { page } from '$app/state';
	import { locale } from './lang';
	import { PageState } from './page.svelte.ts';
	import SearchResultsSidebar from './SearchResultsSidebar.svelte';

	const pageState = new PageState();
	void pageState;

	const query = $derived(page.url.searchParams.get('query') ?? '');
</script>

<svelte:head>
	<title>{locale.tabTitle(query)}</title>
</svelte:head>

<SearchResultsSidebar {query} />
