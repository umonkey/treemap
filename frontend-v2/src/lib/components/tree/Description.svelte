<script>
	const { text } = $props();
</script>

<div class="description">
	{#if text}
		<p>{text}</p>
	{:else}
		<p>
			There is no description for this tree. You can leave a comment to let us know about something
			interesting.
		</p>
	{/if}
</div>

<style>
	.description {
		padding: 0 var(--gap);
		border-top: 1px solid var(--sep-color);

		p {
			white-space: pre-wrap;
		}
	}
</style>
