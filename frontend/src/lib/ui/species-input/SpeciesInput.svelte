<script lang="ts">
	/**
	 * This is a complex components that implements text input for
	 * tree species, including autocomplete based on the API vocabulary,
	 * and a history of recent inputs, also backed by the API.
	 */

	import { apiClient } from '$lib/api';
	import { loadSuggestedSpecies } from '$lib/hooks';
	import { locale } from '$lib/locale';
	import type { ISpecies } from '$lib/types';
	import { FormElement } from '$lib/ui';
	import { onMount } from 'svelte';

	const { value = '', onChange } = $props<{
		value?: string | null;
		onChange: (value: string) => void;
	}>();

	// This is the editable input value.
	// We change it on autocomplete clicks, etc.
	let currentValue = $state<string>(value ?? '');

	const { data: suggested, reload } = loadSuggestedSpecies();

	let options: ISpecies[] = $state([]);
	let showOptions = $state<boolean>(false);

	const handleInput = (event: Event) => {
		const target = event.target as HTMLInputElement;

		apiClient.searchSpecies(target.value).then((res) => {
			if (res.status === 200 && res.data) {
				options = res.data;
				showOptions = options.length > 0;
			}
		});
	};

	// This is called when the user clicks an autocomplete suggestion.
	const handleOptionClick = (e: Event, v: string) => {
		e.preventDefault();
		if (e.target instanceof HTMLElement) {
			e.target.blur();
		}

		showOptions = false;
		currentValue = v;
		onChange(v);
	};

	const handleSuggestionClick = (e: Event, v: string) => {
		e.preventDefault();
		if (e.target instanceof HTMLElement) {
			e.target.blur();
		}

		showOptions = false;
		currentValue = v;
		onChange(v);
	};

	const handleFocusOut = () => {
		setTimeout(() => {
			showOptions = false;
		}, 200);
	};

	const handleChange = (e: Event) => {
		if (e.target) {
			const input = e.target as HTMLInputElement;
			onChange(input.value ?? '');
		}
	};

	onMount(() => reload());

	$effect(() => {
		currentValue = value;
	});
</script>

<FormElement label={locale.speciesLabel()} hint={locale.speciesHint()}>
	<label class:drop={showOptions}>
		<input
			type="text"
			autocomplete="off"
			value={currentValue}
			placeholder={locale.speciesPrompt()}
			oninput={handleInput}
			onfocusout={handleFocusOut}
			onchange={handleChange}
		/>
	</label>

	{#if showOptions && options.length > 0}
		<ul class="options" aria-label="suggestions">
			{#each options as option}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<li onclick={(e) => handleOptionClick(e, option.name)}>
					{option.name} <small>~ {option.local}</small>
				</li>
			{/each}
		</ul>
	{/if}

	{#if $suggested && $suggested.length > 0}
		<div class="suggested">
			{#each $suggested as option}
				<button onclick={(e) => handleSuggestionClick(e, option)} type="button">{option}</button>
			{/each}
		</div>
	{/if}
</FormElement>

<style>
	label {
		display: block;
	}

	input {
		width: 100%;
		padding: var(--gap);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		box-sizing: border-box;
		background-color: transparent;
		border: 1px solid var(--sep-color);
		border-radius: 6px;
		color: var(--form-color);
		outline: none;
		line-height: 1.25em;
	}

	.drop input {
		border-bottom-left-radius: 0;
		border-bottom-right-radius: 0;
		border-bottom: none;
	}

	.options {
		list-style-type: none;
		margin: 0;
		padding: 0;
		border: solid 1px var(--sep-color);
		border-bottom: none;
		background-color: var(--form-background);
		border-bottom-left-radius: 6px;
		border-bottom-right-radius: 6px;
		overflow: hidden;

		li {
			padding: var(--gap);
			border-bottom: solid 1px var(--sep-color);
			cursor: pointer;

			&:hover {
				background-color: var(--bg-color-hover);
			}
		}
	}

	small {
		opacity: 0.5;
	}

	.suggested {
		padding: 0;
		margin: 1rem 0 0;

		button {
			border: 1px solid transparent;
			padding: 4px 8px;
			margin: 0 0.5rem 0.5rem 0;
			cursor: pointer;
			background-color: var(--sep-color);

			&:hover {
				border: 1px solid var(--text-color-inactive);
			}
		}
	}
</style>
