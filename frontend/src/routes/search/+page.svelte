<script lang="ts">
	import { locale } from '$lib/locale';
	import { SearchForm } from '$lib/forms';
	import { Header } from '$lib/ui';
</script>

<svelte:head>
	<title>{locale.searchTitle()}</title>
</svelte:head>

<Header title={locale.searchTitle()} />

<SearchForm />
