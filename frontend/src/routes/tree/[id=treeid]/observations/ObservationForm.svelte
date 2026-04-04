<script lang="ts">
	import { goto } from '$app/navigation';
	import { addObservations, getObservations } from '$lib/api/observations';
	import { getUser as getUserApi } from '$lib/api/users';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { showError } from '$lib/errors';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { isAuthenticated } from '$lib/stores/authStore';
	import { addUsers, getUser } from '$lib/stores/userStore';
	import type { IObservation } from '$lib/types';
	import { CheckInput, HelpButton } from '$lib/ui';
	import { onMount } from 'svelte';

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
		bug_holes: false,
		topping: false,
		fungal_bodies: false,
		vfork: false,
		cavities: false,
		vines: false,
		inclined: false,
		nests: false,
		nesting_boxes: false
	});

	const user = $derived($getUser(observation.created_by));
	const date = $derived(
		observation.created_at > 0
			? new Date(observation.created_at * 1000).toISOString().split('T')[0]
			: ''
	);

	onMount(async () => {
		const response = await getObservations(id);
		if (response.data) {
			observation = { ...observation, ...response.data };

			if (
				observation.created_by &&
				observation.created_by !== '0' &&
				!$getUser(observation.created_by)
			) {
				getUserApi(observation.created_by).then((res) => {
					if (res.data) {
						addUsers([res.data]);
					}
				});
			}
		}
		loading = false;
	});

	async function handleSubmit() {
		saving = true;
		const response = await addObservations(observation);
		if (response.error) {
			showError(`Error ${response.status} adding observations: ${response.error.description}`);
		} else {
			goto(routes.mapPreview(id));
		}
		saving = false;
	}

	const handleCancel = () => {
		goto(routes.mapPreview(id));
	};

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

{#if loading}
	<p>Loading...</p>
{:else}
	<TreeForm {id} title="Observations" onSubmit={handleSubmit} onCancel={handleCancel} {saving}>
		<div class="observation-status">
			{#if observation.created_at > 0 && observation.created_by}
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
								bind:value={observation[field.id as keyof IObservation] as boolean}
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
