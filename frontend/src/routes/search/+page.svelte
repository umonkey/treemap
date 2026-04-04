<script lang="ts">
	import { goto } from '$app/navigation';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	import Form from '$lib/ui/form/Form.svelte';
	import SpeciesInput from '$lib/ui/species-input/SpeciesInput.svelte';
	import StreetInput from '$lib/ui/street-input/StreetInput.svelte';
	import { hooks } from './hooks';

	let query = $state<string>('');

	const { handleStreetChange, handleSpeciesChange, handleSubmit } = hooks();

	const handleSearch = (value: string) => {
		goto(routes.searchQuery(value));
	};

	const handleInput = (value: string) => {
		query = value;
	};

	const buttons = [
		{
			title: 'Search',
			onClick: handleSubmit
		}
	];
</script>

<Dialog title={locale.searchTitle()} {buttons}>
	<SearchBar value={query} onInput={handleInput} onSearch={handleSearch} />

	<div>
		{#if query}
			<p>
				<a href={routes.searchQuery(query)} data-testid="search-link">{locale.searchLink(query)}</a>
			</p>
			<hr />
		{/if}

		{#if locale.lang === 'ru'}
			<p>Некоторые запросы для примера:</p>

			<ul>
				<li>
					Есть лазер? <a href={routes.searchQuery('no:diameter')}>Без диаметра</a> или
					<a href={routes.searchQuery('no:height')}>высоты</a>.
				</li>
				<li>
					Есть сантиметр? <a href={routes.searchQuery('no:circumference')}>Без обхвата ствола</a>.
				</li>
				<li>Есть камера? <a href={routes.searchQuery('noimage')}>Без фотографий</a>.</li>
				<li>
					Есть саженец? <a href={routes.searchQuery('gone')}>Пустые места</a> или
					<a href={routes.searchQuery('stump')}>пни</a>.
				</li>
				<li>
					<a href={routes.searchQuery('unknown hasimage')}>Неопознанные деревья</a> с фотографиями
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
					<a href={routes.searchQuery('dead')}>Dead</a>,
					<a href={routes.searchQuery('gone')}>gone</a>
					trees, or <a href={routes.searchQuery('stump')}>stumps</a>.
				</li>
				<li>
					Have a laser? <a href={routes.searchQuery('no:diameter')}>No crown diameter</a> or
					<a href={routes.searchQuery('no:height')}>no height</a>.
				</li>
				<li>
					Have a tape measure? <a href={routes.searchQuery('no:circumference')}
						>No trunk circumference</a
					>.
				</li>
				<li>Have a camera? <a href={routes.searchQuery('noimage')}>No photos</a>.</li>
				<li>
					Have a tree? <a href={routes.searchQuery('gone')}>Empty spots</a> or
					<a href={routes.searchQuery('stump')}>stumps</a>.
				</li>
				<li>
					With <a href={routes.searchQuery('unknown hasimage')}>unknown species</a> but has photos.
				</li>
				<li><a href={routes.searchQuery('incomplete')}>Missing some measurements</a>.</li>
			</ul>
		{/if}
	</div>

	<Form onSubmit={handleSubmit}>
		<SpeciesInput onChange={handleSpeciesChange} />
		<StreetInput onChange={handleStreetChange} />
	</Form>
</Dialog>

<style>
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
