<script lang="ts">
	import { TreeListLogic } from './TreeList.svelte.ts';
	import { lang } from './lang';
	import Button from '$lib/ui/button/Button.svelte';

	const state = new TreeListLogic();
</script>

{#if state.trees.length > 0}
	<div class="tree-list">
		<h3>{lang.recordedTrees}</h3>
		<ul>
			{#each state.trees as tree, i}
				<li>
					<span class="tree-label">Tree #{i + 1}</span>
					<Button type="secondary" onClick={() => state.removeTree(tree.id)}>
						{lang.removeTree}
					</Button>
				</li>
			{/each}
		</ul>
	</div>
{/if}

<style>
	.tree-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;

		h3 {
			margin: 0;
			font-size: 1rem;
			font-weight: 600;
		}

		ul {
			margin: 0;
			padding: 0;
			list-style: none;
			display: flex;
			flex-direction: column;
			gap: 0.5rem;
		}

		li {
			display: flex;
			justify-content: space-between;
			align-items: center;
			font-size: 0.95rem;
		}

		.tree-label {
			font-family: monospace;
		}
	}
</style>
