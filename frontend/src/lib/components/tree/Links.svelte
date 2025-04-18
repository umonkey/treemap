<script lang="ts">
	import { formatLinks } from '$lib/utils/trees';
	import type { ITree } from '$lib/types';

	const { tree } = $props<{
		tree: ITree;
	}>();

	const links = formatLinks(tree);
</script>

{#if links.length > 0}
	<ul class="links">
		{#each links as link}
			<li><a href={link.url} target="_blank">{link.text}</a></li>
		{/each}
	</ul>
{/if}

<style>
	.links {
		display: flex;
		gap: var(--gap);
		list-style: none;
		padding: 0 var(--gap);
	}
</style>
