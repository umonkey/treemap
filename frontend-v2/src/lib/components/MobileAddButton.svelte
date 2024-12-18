<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { routes } from '$lib/routes';

	import PlusIcon from '$lib/icons/PlusIcon.svelte';

	const isHidden = () => {
		const path = $page.url.pathname;

		if (path === '/add') {
			return true;
		}

		return false;
	};

	const handleClick = () => {
		goto(routes.treeAdd());
	};
</script>

<button onclick={handleClick} class:hidden={isHidden()}>
	<PlusIcon />
</button>

<style>
	button {
		display: none;
	}

	button.hidden {
		display: none;
	}

	@media (max-width: 480px) {
		button {
			display: block;
			position: fixed;
			bottom: 140px;
			right: 10px;

			border: none;
			border-radius: 50%;
			outline: none;
			padding: 2px;
			cursor: pointer;

			width: 40px;
			height: 40px;

			color: var(--form-border);
			background-color: #fff;
			z-index: var(--z-add-button);
		}
	}
</style>
