<script lang="ts">
	import { getContext } from 'svelte';
	import { page } from '$app/state';
	import Observations from '$lib/components/observation/Observations.svelte';
	import Comment from '$lib/components/tree/Comment.svelte';
	import { handleShareTree } from '$lib/hooks/shareTree';
	import SettingsIcon from '$lib/icons/SettingsIcon.svelte';
	import ShareIcon from '$lib/icons/ShareIcon.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import Actions from '../components/Actions.svelte';
	import Properties from '../components/Properties.svelte';
	import { TREE_CONTEXT_KEY, type TreeLayoutState } from '../layout.svelte.ts';
	import { previewContentState } from './PreviewContent.svelte.ts';
	import '$lib/styles/variables.css';

	const layoutState = getContext<TreeLayoutState>(TREE_CONTEXT_KEY);

	const id = $derived(page.params.id as string);

	$effect(() => {
		return previewContentState.init();
	});

	$effect(() => {
		previewContentState.reload(id);
	});
</script>

{#if layoutState.tree}
	{@const tree = layoutState.tree}

	<Properties {tree} />
	<hr/>
	<Actions {tree} />

	<Buttons>
		<Button type="secondary" onClick={() => handleShareTree(tree.id)} square><ShareIcon /></Button>
		<Button
			type="secondary"
			onClick={(e) => previewContentState.handleContextMenu(tree.id, e)}
			square
		>
			<SettingsIcon />
		</Button>
	</Buttons>

	<div class="extras">
		<Observations observation={previewContentState.observations} />

		{#each previewContentState.comments as comment}
			<Comment {comment} />
		{/each}
	</div>
{/if}

<style>
	.extras {
		display: flex;
		flex-direction: column;
		gap: var(--gap);
	}

	hr {
		margin: 0;
		padding: 0;
		border: none;
		border-bottom: 1px solid rgba(128, 128, 128, 0.2);
	}
</style>
