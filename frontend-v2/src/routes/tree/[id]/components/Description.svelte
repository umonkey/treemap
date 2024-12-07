<script>
	const { text } = $props();
</script>

<div class="description">
	<p>{text}</p>
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
