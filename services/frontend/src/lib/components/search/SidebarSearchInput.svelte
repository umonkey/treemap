<script lang="ts">
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import SearchIcon from '$lib/icons/SearchIcon.svelte';
	import { locale } from '$lib/locale';

	const {
		value,
		onInput,
		onSearch,
		onClose
	}: {
		value: string;
		onInput: (query: string) => void;
		onSearch: (query: string) => void;
		onClose: () => void;
	} = $props();

	const handleInput = (event: Event) => {
		onInput((event.target as HTMLInputElement).value);
	};

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.key === 'Enter') {
			event.preventDefault();
			onSearch(value);
		}
	};
</script>

<div class="sidebar-search-input">
	<input
		type="search"
		data-testid="sidebar-search-input"
		placeholder={locale.searchPrompt()}
		{value}
		oninput={handleInput}
		onkeydown={handleKeyDown}
	/>
	<button
		type="button"
		class="icon-btn"
		aria-label={locale.sideSearch()}
		disabled={!value.trim()}
		onclick={() => onSearch(value)}
	>
		<SearchIcon width="20px" height="20px" />
	</button>
	<button type="button" class="icon-btn close" aria-label={locale.editCancel()} onclick={onClose}>
		<CloseIcon />
	</button>
</div>

<style>
	.sidebar-search-input {
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: 0;

		background-color: rgba(128, 128, 128, 0.5);
		border-radius: 8px;

		:global(svg) {
			width: 20px;
			height: 20px;
		}

		input {
			flex: 1 1 auto;
			min-width: 0;
			height: 40px;
			box-sizing: border-box;
			padding: 0 10px;
			margin: 0 10px 0 0;

			background-color: transparent;
			color: var(--text-color);
			border: none;
			font: inherit;

			&:focus {
				outline: none;
			}
		}

		.icon-btn {
			flex: 0 0 auto;
			display: flex;
			align-items: center;
			justify-content: center;

			width: 40px;
			height: 40px;
			padding: 0;
			cursor: pointer;

			background-color: transparent;
			border: none;
			color: light-dark(black, white);
			opacity: 0.5;

			&:not(:disabled):hover {
				opacity: 1;
			}

			&:disabled {
				opacity: 0.25;
				cursor: default;
			}
		}
	}
</style>
