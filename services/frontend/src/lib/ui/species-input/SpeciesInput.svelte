<script lang="ts">
	/**
	 * This is a complex components that implements text input for
	 * tree species, including autocomplete based on the API vocabulary,
	 * and a history of recent inputs, also backed by the API.
	 */

	import { locale } from '$lib/locale';
	import SelectButton from '$lib/ui/SelectButton.svelte';
	import FormElement from '$lib/ui/form-element/FormElement.svelte';
	import { onMount } from 'svelte';
	import { SpeciesInputLogic } from './SpeciesInput.svelte.ts';

	const {
		value = '',
		nosuggestions = false,
		onChange
	}: {
		value?: string | null;
		nosuggestions?: boolean;
		onChange: (value: string) => void;
	} = $props();

	const componentState = new SpeciesInputLogic();

	onMount(() => {
		if (!nosuggestions) {
			componentState.loadSuggested();
		}
	});

	$effect(() => {
		componentState.syncValue(value);
	});
</script>

<FormElement label={locale.speciesLabel()} hint={locale.speciesHint()}>
	<label class:drop={componentState.showOptions}>
		<input
			type="text"
			autocomplete="off"
			value={componentState.currentValue}
			placeholder={locale.speciesPrompt()}
			oninput={componentState.handleInput}
			onfocusout={componentState.handleFocusOut}
			onchange={(e) => componentState.handleChange(e, onChange)}
		/>
	</label>

	{#if componentState.showOptions && componentState.options.length > 0}
		<ul class="options" aria-label="suggestions">
			{#each componentState.options as option}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<li onclick={(e) => componentState.handleOptionClick(e, option.name, onChange)}>
					{option.name} <small>~ {option.local}</small>
				</li>
			{/each}
		</ul>
	{/if}

	{#if !nosuggestions && componentState.suggested && componentState.suggested.length > 0}
		<div class="suggested">
			{#each componentState.suggested as option}
				{#if option}
					<SelectButton
						value={option}
						label={option}
						onClick={(v) => componentState.handleSuggestionClick(v, onChange)}
					/>
				{/if}
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

		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		gap: 0.5rem;
	}
</style>
