<script lang="ts">
	import { ToolListLogic } from './ToolList.svelte.ts';

	const componentState = new ToolListLogic();
</script>

<h2>Available tools</h2>

{#if componentState.items.length > 0}
	<ul>
		{#each componentState.items as item}
			<li><a href={item.href}>{item.name}</a></li>
		{/each}
	</ul>
{:else}
	<p>No extra tools are available to you at this moment.</p>
{/if}

<style>
	h2 {
		margin-top: 40px;
		font-weight: 400;
		font-size: 20px;
		opacity: 0.75;
	}

	ul {
		line-height: 1.5em;
		margin-top: 10px;
	}

	p {
		margin-top: 10px;
		opacity: 0.75;
	}
</style>
