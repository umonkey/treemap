<script lang="ts">
	/**
	 * This is a complex components that implements text input for
	 * tree species, including autocomplete based on the API vocabulary,
	 * and a history of recent inputs, also backed by the API.
	 */

	import { onMount } from 'svelte';
	import { apiClient } from '$lib/api';
	import type { ISpecies } from '$lib/types';
	import { locale } from '$lib/locale';
	import { loadSuggestedSpecies } from '$lib/hooks';

	let { value = $bindable() } = $props();
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

	const handleOptionClick = (selectedValue: string) => {
		showOptions = false;
		value = selectedValue;
	};

	const handleSuggestionClick = (sug: string) => {
		value = sug;
	};

	const handleFocusOut = () => {
		setTimeout(() => {
			showOptions = false;
		}, 200);
	};

	onMount(() => reload());
</script>

<div class="input">
	<label class:drop={showOptions}>
		<span>{locale.speciesLabel()}</span>
		<input
			type="text"
			autocomplete="off"
			bind:value
			placeholder={locale.speciesPrompt()}
			oninput={handleInput}
			onfocusout={handleFocusOut}
		/>

		{#if showOptions}
			<ul class="options">
				{#each options as option}
					<!-- svelte-ignore a11y_click_events_have_key_events -->
					<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
					<li onclick={() => handleOptionClick(option.name)}>
						{option.name} <small>~ {option.local}</small>
					</li>
				{/each}
			</ul>
		{/if}
	</label>

	{#if $suggested}
		<ul class="suggested">
			{#each $suggested as option}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<li onclick={() => handleSuggestionClick(option)}><u>{option}</u></li>
			{/each}
		</ul>
	{/if}

	<div class="hint">{locale.speciesHint()}</div>
</div>

<style>
	label {
		display: block;
		margin-top: calc(2 * var(--gap));
	}

	span {
		display: block;
		margin-bottom: var(--gap);
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

	.hint {
		color: var(--text-color-inactive);
		font-size: 0.85em;
		line-height: 125%;
		margin-top: var(--gap);
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

	ul.suggested {
		padding: 0;
		list-style-type: none;

		li {
			display: inline-block;
			opacity: 0.5;
			margin-right: 0.25em;

			u {
				cursor: pointer;
				color: var(--link-color);
			}

			&:after {
				content: ',';
			}

			&:last-child:after {
				content: '';
			}
		}
	}
</style>
