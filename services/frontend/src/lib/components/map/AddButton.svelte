<script lang="ts">
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
			<svg
				class="add-icon"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
				aria-hidden="true"
			>
				<path d="M12 5v14M5 12h14" />
			</svg>
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
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
		transition: box-shadow 0.15s ease;
	}

	button.add-button:not(:disabled):hover,
	button.add-button:not(:disabled):active {
		background-color: var(--map-primary-background);
	}

	.add-button:hover {
		box-shadow: 0 4px 10px rgba(0, 0, 0, 0.35);
	}

	.add-button .add-icon {
		width: 24px;
		height: 24px;
	}
</style>
