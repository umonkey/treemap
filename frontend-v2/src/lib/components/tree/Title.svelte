<script>
	import DotsIcon from '$lib/icons/DotsIcon.svelte';
	import { menuState } from '$lib/stores/treeMenu';

	const { title, address = undefined } = $props();

	const onMenu = () => {
		menuState.update((value) => !value);
	};
</script>

<div class="title">
	<h1>
		<strong>{title}</strong>{#if address}
			&middot; {address}{/if}
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

		h1 {
			font-size: 1rem;
			font-weight: 400;
			line-height: 50px;
			margin: 0;
		}

		.edit {
			flex-basis: 25px;
			border: none;
			background-color: transparent;
			cursor: pointer;
			color: var(--icon-color-secondary);
		}
	}
</style>
