<script lang="ts">
	import type { IChange } from '$lib/types';
	import { formatDate } from '$lib/utils/strings';
	import { get } from 'svelte/store';
	import { getUser } from '$lib/stores/userStore';

	const { changes } = $props<{
		changes: IChange[];
	}>();
</script>

<div class="change-list">
	{#if changes.length > 0}
		<dl>
			{#each changes as change}
				<dt>{formatDate(change.added_at)}, {get(getUser)(change.added_by).name}:</dt>
				<dd>{change.name} → {change.value}</dd>
			{/each}
		</dl>
	{:else}
		<p>No changes found for this tree.</p>
	{/if}
</div>

<style>
	dl {
		margin: var(--gap) 0;
	}

	dt {
		margin: 5px 0;
	}

	dd {
		margin: 0 0 var(--gap) 30px;
	}
</style>
