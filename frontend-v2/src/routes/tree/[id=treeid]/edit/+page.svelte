<script>
	import { apiClient } from '$lib/api';
	import { goto } from '$app/navigation';
	import { routes } from '$lib/routes';
	import { toast } from '@zerodevx/svelte-toast';

	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import SpeciesInput from '$lib/components/forms/SpeciesInput.svelte';
	import HeightInput from '$lib/components/forms/HeightInput.svelte';
	import CanopyInput from '$lib/components/forms/CanopyInput.svelte';
	import CircumferenceInput from '$lib/components/forms/CircumferenceInput.svelte';
	import StateInput from '$lib/components/forms/StateInput.svelte';
	import AddressInput from '$lib/components/forms/AddressInput.svelte';
	import NotesInput from '$lib/components/forms/NotesInput.svelte';
	import Button from '$lib/components/forms/Button.svelte';
	import Header from '$lib/components/tree/Header.svelte';

	const { data } = $props();
	const treeId = data.id;

	let species = $state(data.tree.species ?? '');
	let height = $state(data.tree.height ?? 0);
	let canopy = $state(data.tree.canopy ?? 0);
	let circumference = $state(data.tree.circumference ?? 0);
	let state = $state(data.tree.state ?? '');
	let address = $state('');
	let notes = $state(data.tree.notes ?? '');

	$effect(() => {
		console.debug('Species changed to:', species);
	});

	$effect(() => {
		console.debug('Address changed to:', address);
	});

	const onSave = () => {
		apiClient
			.updateTree(treeId, {
				species,
				height,
				canopy,
				circumference,
				state,
				address,
				notes
			})
			.then((res) => {
				if (res.status >= 200 && res.status < 400) {
					goto(routes.treeDetails(treeId));
				} else {
					console.error(`Error ${res.status} updating tree.`);
					toast.push('Error updating tree.');
				}
			})
			.catch((e) => {
				console.error(`Error updating tree: ${e}.`);
				toast.push('Error updating tree.');
			});
	};

	const onCancel = () => {
		history.back();
	};
</script>

<Header title="Edit tree" />

<div class="form">
	<AuthWrapper>
		<SpeciesInput bind:value={species} />
		<HeightInput bind:value={height} />
		<CanopyInput bind:value={canopy} />
		<CircumferenceInput bind:value={circumference} />
		<StateInput bind:value={state} />
		<AddressInput bind:value={address} />
		<NotesInput bind:value={notes} />

		<div class="buttons">
			<Button type="submit" label="Save changes" onClick={onSave} />
			<Button type="cancel" label="Cancel" onClick={onCancel} />
		</div>
	</AuthWrapper>
</div>

<style>
	.form {
		padding: 0 var(--gap) var(--gap);
	}

	.buttons {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
		margin-top: var(--gap);
	}
</style>
