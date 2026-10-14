<script lang="ts">
	import { locale } from './TopUsers.lang';
	import { TopUsers, formatCount } from './TopUsers.svelte.ts';

	const componentState = new TopUsers();

	$effect(() => {
		componentState.init();
	});
</script>

<div>
	{#if componentState.loading && componentState.users.length === 0}
		<!-- loading -->
	{:else if componentState.error}
		<p>Error loading contributors: {componentState.error}</p>
	{:else if componentState.users.length > 0}
		<div class="grid">
			{#each componentState.users as item (item.id)}
				<div class="cell">
					{#if item.picture}
						<img src={item.picture} alt="" referrerpolicy="no-referrer" class="user-pic" />
					{:else}
						<div class="user-pic"></div>
					{/if}
					<span class="name" title={item.name}>{item.name}</span>
					<span class="count">{formatCount(item.count)}</span>
				</div>
			{/each}
		</div>
	{:else}
		<p>{locale.noRecentContributions()}</p>
	{/if}
</div>

<style>
	.grid {
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		gap: 1rem;
	}

	.cell {
		display: grid;
		grid-template-rows: auto auto auto;
		justify-items: center;
		gap: 0.5rem;
		min-width: 0;
		text-align: center;
		line-height: 1.5em;
	}

	.user-pic {
		width: 100%;
		aspect-ratio: 1;
		border-radius: 50%;
		object-position: center;
		object-fit: cover;
		overflow: hidden;
	}

	.name {
		width: 100%;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-weight: 500;
	}

	.count {
		color: var(--pico-muted-color, #666);
		font-size: 0.9em;
		white-space: nowrap;
	}
</style>
