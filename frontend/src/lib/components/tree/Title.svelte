<script>
	import DotsIcon from '$lib/icons/DotsIcon.svelte';
	import { menuState } from '$lib/stores/treeMenu';
	import { routes } from '$lib/routes';

	const { title, address = undefined } = $props();

	const onMenu = () => {
		menuState.update((value) => !value);
	};
</script>

<div class="title">
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
	.title {
		display: flex;
		justify-content: space-between;
		align-items: center;
		height: 50px;
		padding: 0 var(--gap);
		gap: var(--gap);

		h1 {
			font-size: 1rem;
			font-weight: 400;
			line-height: 50px;
			margin: 0;

			white-space: nowrap;
			overflow: hidden;
			text-overflow: ellipsis;
		}

		.edit {
			flex-basis: 25px;
			flex-shrink: 0;
			flex-grow: 0;
			border: none;
			background-color: transparent;
			cursor: pointer;
			color: var(--icon-color-secondary);
		}
	}
</style>
