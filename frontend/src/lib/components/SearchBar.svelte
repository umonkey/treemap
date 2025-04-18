<script lang="ts">
	import { SearchIcon } from '$lib/icons';
	import { locale } from '$lib/locale';

	const { value, onInput, onSearch } = $props<{
		value: string;
		onInput: (query: string) => void;
		onSearch: (query: string) => void;
	}>();

	const handleInput = (event: Event) => {
		const input = event.target as HTMLInputElement;
		const newValue = input.value;
		onInput(newValue);
	};

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.key === 'Enter') {
			onSearch(value);
		}
	};
</script>

<div class="search">
	<div class="icon"><SearchIcon width="20px" height="20px" /></div>
	<!-- svelte-ignore a11y_autofocus -->
	<input
		type="search"
		data-testid="search-input"
		placeholder={locale.searchPrompt()}
		autofocus
		value
		oninput={handleInput}
		onkeydown={handleKeyDown}
	/>
</div>

<style>
	div.search {
		background-color: var(--form-background);
		display: flex;
		flex-direction: row;
		height: 40px;
		border-radius: 8px;
	}

	.icon {
		height: 40px;
		width: 40px;
		align-content: center;
		text-align: center;
		color: var(--icon-color-secondary);
		padding-top: 4px;
		box-sizing: border-box;

		flex-basis: 40px;
		flex-shrink: 0;
		flex-grow: 0;
	}

	input {
		flex-grow: 1;

		background-color: transparent;
		border: none;
		color: #fff;
		height: 40px;
		line-height: 40px;

		&:focus {
			outline: none;
		}
	}
</style>
