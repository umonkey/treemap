<script lang="ts">
	import { page } from '$app/state';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { locale } from '$lib/locale';
	import { isAuthenticated } from '$lib/stores/authStore';
	import { getUser } from '$lib/stores/userStore';
	import type { IObservation } from '$lib/types';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import HelpButton from '$lib/ui/help-button/HelpButton.svelte';
	import { pageState } from './page.svelte';

	const id = $derived(page.params.id);

	$effect(() => {
		if (id) {
			pageState.reload(id);
		}
	});

	const user = $derived($getUser(pageState.observation.created_by));
	const date = $derived(
		pageState.observation.created_at > 0
			? new Date(pageState.observation.created_at * 1000).toISOString().split('T')[0]
			: ''
	);

	const fields = [
		{ id: 'bark_damage', label: locale.observationBarkDamage() },
		{ id: 'dry_branches', label: locale.observationDryBranches() },
		{ id: 'leaking', label: locale.observationLeaking() },
		{ id: 'root_damage', label: locale.observationRootDamage() },
		{ id: 'open_roots', label: locale.observationOpenRoots() },
		{ id: 'bug_holes', label: locale.observationBugHoles() },
		{ id: 'topping', label: locale.observationTopping() },
		{ id: 'fungal_bodies', label: locale.observationFungalBodies() },
		{ id: 'vfork', label: locale.observationVfork() },
		{ id: 'cavities', label: locale.observationCavities() },
		{ id: 'vines', label: locale.observationVines() },
		{ id: 'inclined', label: locale.observationInclined() },
		{ id: 'nests', label: locale.observationNests() },
		{ id: 'nesting_boxes', label: locale.observationNestingBoxes() }
	];

	const HELP_URL = 'https://github.com/umonkey/treemap/wiki/Defects';
</script>

{#if pageState.loading}
	<p>Loading...</p>
{:else}
	<TreeForm
		{id}
		title="Observations"
		onSubmit={pageState.handleSubmit}
		onCancel={pageState.handleCancel}
		saving={pageState.saving}
	>
		<div class="observation-status">
			{#if pageState.observation.created_at > 0 && pageState.observation.created_by}
				<p>Last observation made on {date} by {user?.name ?? 'unknown user'}</p>
			{:else}
				<p>There are no observations for this tree, add the first one.</p>
			{/if}
		</div>

		<table class="observation-table">
			<tbody>
				{#each fields as field}
					<tr>
						<td class="checkbox-cell">
							<CheckInput
								id={field.id}
								disabled={!$isAuthenticated}
								bind:value={pageState.observation[field.id as keyof IObservation] as boolean}
							/>
						</td>
						<td class="label-cell">
							<label for={field.id}>{field.label}</label>
						</td>
						<td class="help-cell">
							<HelpButton help={HELP_URL} />
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</TreeForm>
{/if}

<style>
	.observation-table {
		width: 100%;
		border-collapse: collapse;
		margin-bottom: var(--gap-large);

		tr {
			border: none;
		}

		td {
			padding: calc(var(--gap) / 2) 0;
			vertical-align: middle;
		}
	}

	.checkbox-cell {
		width: 50px;
	}

	.label-cell {
		label {
			cursor: pointer;
			display: block;
			width: 100%;
		}
	}

	.help-cell {
		width: 40px;
		text-align: right;
	}

	.observation-status {
		margin-bottom: var(--gap-large);
		color: var(--text-color-secondary);
		font-size: 0.9em;
	}
</style>
