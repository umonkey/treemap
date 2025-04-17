<script>
import { formatLinks } from "$lib/utils/trees";

const { tree } = $props();
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
