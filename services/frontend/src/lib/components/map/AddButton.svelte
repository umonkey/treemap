<script lang="ts">
	import PlusIcon from '$lib/icons/PlusIcon.svelte';
	import { locale } from '$lib/locale';
	import { hasPermission } from '$lib/stores/authStore';
	import { Control, getMapContext } from 'svelte-maplibre';
	import { AddButtonLogic } from './AddButton.svelte.ts';

	const componentState = new AddButtonLogic();
	const mapContext = getMapContext();
</script>

{#if $hasPermission('tree:create') || $hasPermission('water:manage')}
	<Control position="bottom-right">
		<button
			class="add-button"
			type="button"
			title={locale.addButton()}
			onclick={() => mapContext.map && componentState.handleClick(mapContext.map)}
		>
			<PlusIcon />
		</button>
	</Control>
{/if}

<style>
	.add-button {
		box-sizing: border-box;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		padding: 0;
		border: 2px solid #fff;
		border-radius: 50%;
		background-color: var(--map-primary-background);
		color: #fff;
		cursor: pointer;
		box-shadow: 0 0 0 2px rgba(0, 0, 0, 0.1);
		transition: filter 0.15s ease;
	}

	.add-button:hover {
		filter: brightness(1.1);
	}

	.add-button :global(svg) {
		width: 24px;
		height: 24px;
	}
</style>
