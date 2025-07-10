<script lang="ts">
	/**
	 * This is a complex components that implements text input for
	 * street addresses, including autocomplete based on the API vocabulary.
	 */

	import { apiClient } from '$lib/api';
	import { locale } from '$lib/locale';
	import type { IStreet } from '$lib/types';
	import { FormElement } from '$lib/ui';

	const { value, onChange } = $props<{
		value: string;
		onChange: (value: string) => void;
	}>();

	// This is the editable input value.
	// We change it on autocomplete clicks, etc.
	let currentValue = $state<string>(value);

	let options: IStreet[] = $state([]);
	let showOptions = $state<boolean>(false);

	let input: HTMLInputElement = $state();

	const handleInput = (event: Event) => {
		const target = event.target as HTMLInputElement;

		apiClient.searchStreets(target.value).then((res) => {
			if (res.status === 200 && res.data) {
				options = res.data;
				showOptions = options.length > 0;
			}
		});
	};

	// This is called when the user clicks an autocomplete suggestion.
	const handleOptionClick = (v: string) => {
		showOptions = false;
		currentValue = v;
		input.value = v;
		onChange(v);
	};

	const handleFocusOut = () => {
		setTimeout(() => {
			showOptions = false;
		}, 200);
	};

	const handleChange = (e: Event) => {
		if (e.target && !showOptions) {
			const input = e.target as HTMLInputElement;
			onChange(input.value ?? '');
		}
	};

	$effect(() => {
		currentValue = value;
	});
</script>

<FormElement label={locale.streetLabel()} hint={locale.streetHint()}>
	<label class:drop={showOptions}>
		<input
			type="text"
			autocomplete="off"
			value={currentValue}
			placeholder={locale.speciesPrompt()}
			oninput={handleInput}
			onfocusout={handleFocusOut}
			onchange={handleChange}
			bind:this={input}
		/>
	</label>

	{#if showOptions && options.length > 0}
		<ul class="options" aria-label="suggestions">
			{#each options as option}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<li onclick={() => handleOptionClick(option.name)}>
					{option.name}
				</li>
			{/each}
		</ul>
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
</style>
