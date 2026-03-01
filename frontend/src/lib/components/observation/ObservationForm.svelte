<script lang="ts">
	import { onMount } from 'svelte';
	import { locale } from '$lib/locale';
	import { apiClient } from '$lib/api';
	import type { IObservation } from '$lib/types';
	import { Button, Buttons, Form, HelpButton } from '$lib/ui';
	import { toast } from '@zerodevx/svelte-toast';

	const { id } = $props<{ id: string }>();

	let loading = $state(true);
	let saving = $state(false);
	let observation = $state<IObservation>({
		id: '',
		tree_id: id,
		created_at: 0,
		created_by: '',
		bark_damage: false,
		dry_branches: false,
		leaking: false,
		root_damage: false,
		open_roots: false,
		topping: false,
		fungal_bodies: false,
		vfork: false,
		cavities: false,
		vines: false,
		nests: false,
		nesting_boxes: false
	});

	onMount(async () => {
		const response = await apiClient.getObservations(id);
		if (response.data) {
			observation = { ...observation, ...response.data };
		}
		loading = false;
	});

	async function handleSubmit() {
		saving = true;
		const response = await apiClient.addObservations(observation);
		if (response.error) {
			toast.push(response.error.description);
		} else {
			toast.push('Observations saved');
		}
		saving = false;
	}

	const fields = [
		{ id: 'bark_damage', label: locale.observationBarkDamage() },
		{ id: 'dry_branches', label: locale.observationDryBranches() },
		{ id: 'leaking', label: locale.observationLeaking() },
		{ id: 'root_damage', label: locale.observationRootDamage() },
		{ id: 'open_roots', label: locale.observationOpenRoots() },
		{ id: 'topping', label: locale.observationTopping() },
		{ id: 'fungal_bodies', label: locale.observationFungalBodies() },
		{ id: 'vfork', label: locale.observationVfork() },
		{ id: 'cavities', label: locale.observationCavities() },
		{ id: 'vines', label: locale.observationVines() },
		{ id: 'nests', label: locale.observationNests() },
		{ id: 'nesting_boxes', label: locale.observationNestingBoxes() }
	];

	const HELP_URL = 'https://github.com/umonkey/treemap/wiki/Defects';
</script>

{#if loading}
	<p>Loading...</p>
{:else}
	<Form onSubmit={handleSubmit}>
		<table class="observation-table">
			<tbody>
				{#each fields as field}
					<tr>
						<td class="checkbox-cell">
							<input
								type="checkbox"
								id={field.id}
								bind:checked={observation[field.id as keyof IObservation]}
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

		<Buttons>
			<Button type="submit" disabled={saving}>{locale.editSave()}</Button>
		</Buttons>
	</Form>
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
		width: 30px;

		input {
			width: 20px;
			height: 20px;
			cursor: pointer;
			display: block;
		}
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
</style>
