<script>
	import SearchBar from '$lib/components/SearchBar.svelte';
	import { routes } from '$lib/routes';
</script>

<div class="search">
	<h1>Search</h1>

	<SearchBar />

	<p>Some interesting searches:</p>

	<ul>
		<li>
			<a href={routes.search('sick')}>Sick</a> or <a href={routes.search('dead')}>dead</a> or
			<a href={routes.search('gone')}>gone</a>
			trees, or <a href={routes.search('stomp')}>stomps</a>
		</li>
		<li>
			<a href={routes.search('quercus')}>Oaks</a> or <a href={routes.search('acer')}>maples</a> or
			<a href={routes.search('fraxinus')}>ashes</a>
			or <a href={routes.search('elm')}>elms</a>
		</li>
		<li>
			With <a href={routes.search('noimage')}>no images</a> or
			<a href={routes.search('unknown')}>no species</a>
		</li>
		<li><a href={routes.search('incomplete')}>Missing some data</a></li>
	</ul>
</div>

<style>
	.search {
		padding: 0 var(--gap);
	}

	li {
		margin: 0 0 var(--gap);
	}
</style>
