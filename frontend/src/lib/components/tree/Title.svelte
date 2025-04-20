<script lang="ts">
	/**
	 * This is part of the tree page, an additional header
	 * with the tree title and the (...) button for additional
	 * actions.
	 **/

	import { DotsIcon } from '$lib/icons';
	import { routes } from '$lib/routes';
	import { menuState } from '$lib/stores/treeMenu';

	const { title, address = undefined } = $props<{
		title: string;
		address?: string;
	}>();

	const onMenu = () => {
		menuState.update((value) => !value);
	};
</script>

<div class="tree-title">
	<h1>
		<strong>{title}</strong>
		{#if address}
			&middot; <a href={routes.searchAddress(address)}>{address}</a>{/if}
	</h1>
	<button class="edit" onclick={onMenu}>
		<DotsIcon />
	</button>
</div>

<style>
	.tree-title {
		display: flex;
		justify-content: space-between;
		align-items: center;
		height: 50px;
		padding: 0 var(--gap);
		gap: var(--gap);

		h1 {
			flex-grow: 1;

			font-size: 1rem;
			font-weight: 400;
			line-height: 50px;
			margin: 0;

			white-space: nowrap;
			overflow: hidden;
			text-overflow: ellipsis;
		}

		button {
			flex-basis: 50px;
			flex-shrink: 0;
			flex-grow: 0;

			border: none;
			background-color: transparent;
			cursor: pointer;
			color: var(--icon-color-secondary);

			height: 50px;
			width: 50px;
			padding: 10px;
		}
	}
</style>
