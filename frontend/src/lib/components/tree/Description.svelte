<script lang="ts">
	import { locale } from '$lib/locale';

	const { text } = $props<{
		text: string | null;
	}>();
</script>

<div class="description">
	{#if text}
		<p class="formatted">{text}</p>
	{:else}
		<p>{locale.noDescription()}</p>
	{/if}
</div>

<style>
	.description {
		padding: 0 var(--gap);
		border-top: 1px solid var(--sep-color);

		p.formatted {
			white-space: pre-wrap;
		}
	}
</style>
