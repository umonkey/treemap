<script lang="ts">
	import type { IChange } from '$lib/types';
	import { locale } from '$lib/locale';
	import { format } from './hooks';

	const { changes } = $props<{
		changes: IChange[];
	}>();
</script>

<div class="change-list">
	{#if changes.length > 0}
		<dl>
			{#each format(changes) as rec}
				<dt>{rec.header}</dt>
				<dd>{rec.body}</dd>
			{/each}
		</dl>
	{:else}
		<p>{locale.noChangeHistory()}</p>
	{/if}
</div>

<style>
	dl {
		margin: 0;
	}

	dt {
		margin: 5px 0;
	}

	dd {
		margin: 0 0 var(--gap) 30px;
	}
</style>
