<script lang="ts">
	import { goto } from '$app/navigation';
	import { routes } from '$lib/routes';
	import { locale } from '$lib/locale';

	import Header from '$lib/components/tree/Header.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';

	let query = '';

	const onSearch = () => {
		if (query) {
			goto(routes.searchQuery(query));
		}
	};
</script>

<Header title="Search" />

<div class="search">
	<SearchBar bind:value={query} {onSearch} />

	{#if query}
		<p><a href={routes.searchQuery(query)}>{locale.searchLink(query)}</a></p>
		<hr />
	{/if}

	{#if navigator.language === 'ru-RU'}
		<p>Некоторые запросы для примера:</p>

		<ul>
			<li>
				<a href={routes.searchQuery('sick')}>Больные</a> или
				<a href={routes.searchQuery('dead')}>мёртвые</a>
				или
				<a href={routes.searchQuery('gone')}>удалённые</a>
				деревья, или <a href={routes.searchQuery('stomp')}>пни</a>
			</li>
			<li>
				<a href={routes.searchSpecies('quercus')}>Дубы</a> или
				<a href={routes.searchSpecies('acer')}>клёны</a>
				или
				<a href={routes.searchSpecies('fraxinus')}>ясени</a>
				или <a href={routes.searchSpecies('ulmus')}>вязы</a>
			</li>
			<li>
				<a href={routes.searchQuery('unknown hasimage')}>Неопознанные деревья</a> с фотографиями
			</li>
			<li>
				<a href={routes.searchQuery('noimage')}>Без фотографий</a> или
				<a href={routes.searchQuery('unknown')}>неопознанные</a>
			</li>
			<li>Деревья <a href={routes.searchQuery('incomplete')}>без некоторых параметров</a></li>
		</ul>
	{:else}
		<p>Some interesting searches:</p>

		<ul>
			<li>
				<a href={routes.searchQuery('sick')}>Sick</a> or
				<a href={routes.searchQuery('dead')}>dead</a>
				or
				<a href={routes.searchQuery('gone')}>gone</a>
				trees, or <a href={routes.searchQuery('stomp')}>stomps</a>
			</li>
			<li>
				<a href={routes.searchSpecies('quercus')}>Oaks</a> or
				<a href={routes.searchSpecies('acer')}>maples</a>
				or
				<a href={routes.searchSpecies('fraxinus')}>ashes</a>
				or <a href={routes.searchSpecies('ulmus')}>elms</a>
			</li>
			<li>With <a href={routes.searchQuery('unknown hasimage')}>unknown species</a> and photos</li>
			<li>
				With <a href={routes.searchQuery('noimage')}>no images</a> or
				<a href={routes.searchQuery('unknown')}>no species</a>
			</li>
			<li><a href={routes.searchQuery('incomplete')}>Missing some data</a></li>
		</ul>
	{/if}
</div>

<style>
	.search {
		margin-top: var(--gap);
		padding: 0 var(--gap);
	}

	li {
		margin: 0 0 var(--gap);
	}

	hr {
		border: none;
		border-top: 1px solid var(--sep-color);
		height: 0;
		margin: var(--gap) 0;
	}
</style>
