<script lang="ts">
	import { goto } from '$app/navigation';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	import SearchBar from '$lib/components/SearchBar.svelte';

	let query = $state<string>('');

	const handleSearch = (value: string) => {
		goto(routes.searchQuery(value));
	};

	const handleInput = (value: string) => {
		query = value;
	};
</script>

<div class="search">
	<SearchBar value={query} onInput={handleInput} onSearch={handleSearch} />

	{#if query}
		<p>
			<a href={routes.searchQuery(query)} data-testid="search-link">{locale.searchLink(query)}</a>
		</p>
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
				деревья, или <a href={routes.searchQuery('stump')}>пни</a>
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
			<li>
				Без <a href={routes.searchQuery('no:height')}>высоты</a>,
				<a href={routes.searchQuery('no:diameter')}>диаметра</a>,
				<a href={routes.searchQuery('no:circumference')}>обхвата</a>
			</li>
			<li>
				<a href={routes.searchQuery('incomplete')}>incomplete</a> — деревья без некоторых параметров
			</li>
		</ul>
	{:else}
		<p>Some interesting searches:</p>

		<ul>
			<li>
				<a href={routes.searchQuery('sick')}>Sick</a> or
				<a href={routes.searchQuery('dead')}>dead</a>
				or
				<a href={routes.searchQuery('gone')}>gone</a>
				trees, or <a href={routes.searchQuery('stump')}>stumps</a>
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
			<li>
				Without <a href={routes.searchQuery('no:height')}>height</a>,
				<a href={routes.searchQuery('no:diameter')}>crown diameter</a>,
				<a href={routes.searchQuery('no:circumference')}>trunk circumference</a>
			</li>
			<li><a href={routes.searchQuery('incomplete')}>Missing any data</a></li>
		</ul>
	{/if}
</div>

<style>
	.search {
		li {
			margin: 0 0 var(--gap);
		}

		hr {
			border: none;
			border-top: 1px solid var(--sep-color);
			height: 0;
			margin: var(--gap) 0;
		}
	}
</style>
