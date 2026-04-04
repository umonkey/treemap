<script lang="ts">
import { CloseIcon, SearchIcon } from "$lib/icons";
import { SearchState } from "./SearchControl.svelte.ts";

const state = new SearchState();
</script>

<div class="search-container">
	<div class="search-group">
		<div class="search-icon">
			<SearchIcon />
		</div>
		<input
			type="text"
			placeholder="Search trees..."
			bind:value={state.value}
			onkeydown={(e) => e.key === 'Enter' && state.commit()}
			class="search-input"
		/>
		{#if state.value}
			<button type="button" class="clear-button" onclick={state.clear} title="Clear search">
				<CloseIcon />
			</button>
		{/if}
	</div>
</div>

<style>
	.search-container {
		position: absolute;
		top: calc(10px + var(--safe-area-inset-top));
		left: 50px;
		z-index: 10;
	}

	.search-group {
		display: flex;
		align-items: center;
		height: 30px;
		padding: 0 8px;
		background: #fff;
		border-radius: 4px;
		box-shadow: 0 0 0 2px rgba(0, 0, 0, 0.1);
		min-width: 250px;
	}

	.search-icon {
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #666;
		margin-right: 4px;
		flex-shrink: 0;
	}

	.search-input {
		border: none;
		outline: none;
		font-size: 14px;
		width: 100%;
		height: 100%;
		background: transparent;
		color: #333;
	}

	.clear-button {
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: none;
		border: none;
		cursor: pointer;
		color: #999;
		padding: 0;
		flex-shrink: 0;
	}

	.clear-button:hover {
		color: #333;
	}

	:global(.search-icon svg),
	:global(.clear-button svg) {
		width: 16px;
		height: 16px;
	}

	@media screen and (max-width: 600px) {
		.search-container {
			right: 50px;
		}
	}
</style>
