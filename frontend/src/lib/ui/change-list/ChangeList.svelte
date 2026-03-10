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
		{#each format(changes) as rec}
			<div class="record">
				<div class="userpic">
					<img src={rec.picture} alt="userpic" />
				</div>
				<div>
					<div class="header">{rec.header}</div>
					<div class="body">{rec.body}</div>
				</div>
			</div>
		{/each}
	{:else}
		<p>{locale.noChangeHistory()}</p>
	{/if}
</div>

<style>
	.change-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-top: 1rem;
	}

	.record {
		display: flex;
		flex-direction: row;
		gap: 1rem;

		.userpic {
			flex: 0 0 30px;
		}
	}

	img {
		width: 30px;
		height: 30px;
		border-radius: 50%;
		border: 2px solid #fff;
		box-sizing: border-box;
	}

	.body {
		padding-left: 2rem;
	}
</style>
