<script>
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import { shortDetails } from '$lib/utils/trees';

	export let tree;
	export let onClose;
</script>

{#if tree}
	<div class="preview">
		<div class="header">
			<div class="title">{tree.species}</div>
			<button class="close" on:click={onClose}><CloseIcon /></button>
		</div>
		<div class="props">{shortDetails(tree)}</div>
	</div>
{/if}

<style>
	.preview {
		padding: var(--gap);
		line-height: 1.5em;
	}

	.header {
		display: flex;
		flex-direction: row;

		.title {
			flex-grow: 1;
			flex-shrink: 1;
		}

		.close {
			flex-basis: 30px;
			flex-grow: 0;
			flex-shrink: 0;

			width: 30px;
			height: 30px;
			cursor: pointer;
		}
	}

	@media (max-width: 480px) {
		.preview {
			position: fixed;
			bottom: 50px;

			z-index: 5;

			width: 100%;
			box-sizing: border-box;
			background-color: #000;
			border-top-left-radius: 8px;
			border-top-right-radius: 8px;

			padding: var(--gap);
		}
	}
</style>
