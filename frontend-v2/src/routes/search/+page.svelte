<script>
	import { routes } from '$lib/routes';

	import Header from '$lib/components/tree/Header.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
</script>

<Header title="Search" />

<div class="search">
	<SearchBar />

	<p>Some interesting searches:</p>

	<ul>
		<li>
			<a href={routes.searchQuery('sick')}>Sick</a> or <a href={routes.searchQuery('dead')}>dead</a>
			or
			<a href={routes.searchQuery('gone')}>gone</a>
			trees, or <a href={routes.searchQuery('stomp')}>stomps</a>
		</li>
		<li>
			<a href={routes.searchQuery('oak')}>Oaks</a> or
			<a href={routes.searchQuery('maple')}>maples</a>
			or
			<a href={routes.searchQuery('ash')}>ashes</a>
			or <a href={routes.searchQuery('elm')}>elms</a>
		</li>
		<li>
			With <a href={routes.searchQuery('noimage')}>no images</a> or
			<a href={routes.searchQuery('unknown')}>no species</a>
		</li>
		<li><a href={routes.searchQuery('incomplete')}>Missing some data</a></li>
	</ul>
</div>

<style>
	.search {
		margin-top: var(--gap);
		padding: 0 var(--gap);
	}

	li {
		margin: 0 0 var(--gap);
	}
</style>
