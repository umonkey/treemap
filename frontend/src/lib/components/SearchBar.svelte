<script lang="ts">
	import SearchIcon from '$lib/icons/SearchIcon.svelte';

	export let value = '';
	export let onSearch = () => {};

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.key === 'Enter') {
			onSearch();
		}
	};
</script>

<div class="search">
	<div class="icon"><SearchIcon width="20px" height="20px" /></div>
	<!-- svelte-ignore a11y-autofocus -->
	<input type="search" placeholder="Search" autofocus bind:value on:keydown={handleKeyDown} />
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
