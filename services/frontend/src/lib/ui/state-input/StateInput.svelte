<script lang="ts">
	import { locale } from '$lib/locale';
	import SelectButton from '$lib/ui/SelectButton.svelte';
	import HelpButton from '$lib/ui/help-button/HelpButton.svelte';

	const {
		value,
		label = true,
		onChange
	} = $props<{
		value: string;
		label?: boolean;
		onChange: (value: string) => void;
	}>();

	const states = [
		{ id: 'unknown', label: locale.stateUnknown() },
		{ id: 'healthy', label: locale.stateHealthy() },
		{ id: 'dead', label: locale.stateDead() },
		{ id: 'gone', label: locale.stateGone() },
		{ id: 'stump', label: locale.stateStump() },
		{ id: 'replaced', label: locale.stateReplaced() }
	];
</script>

<div class="input form state-input">
	{#if label}<span class="label">{locale.stateLabel()}</span>{/if}

	<div class="group">
		<div class="buttons">
			{#each states as state}
				<SelectButton
					value={state.id}
					label={state.label}
					active={value === state.id}
					onClick={onChange}
				/>
			{/each}
		</div>
		<HelpButton help="https://github.com/KanachYerevan/kb/wiki/Understanding-tree-state" />
	</div>
</div>

<style>
	.state-input {
		display: block;
	}

	.label {
		display: block;
		margin-bottom: var(--gap);
		opacity: 0.75;
		font-size: 14px;
	}

	.group {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
		align-items: flex-start;
	}

	.buttons {
		display: flex;
		flex-wrap: wrap;
		flex-grow: 1;
		gap: 0.5rem;
	}
</style>
