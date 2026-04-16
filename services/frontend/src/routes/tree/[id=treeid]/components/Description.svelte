<script lang="ts">
	const { text } = $props<{
		text: string | null;
	}>();
</script>

{#if text}
	<div class="description">
		<p class="formatted">{text}</p>
	</div>
{/if}

<style>
	.description {
		padding: 0 var(--gap);
		border-top: 1px solid var(--sep-color);

		p.formatted {
			white-space: pre-wrap;
		}
	}
</style>
