<script lang="ts">
	import { apiClient } from '$lib/api';
	import { goto } from '$app/navigation';
	import { routes } from '$lib/routes';
	import { toast } from '@zerodevx/svelte-toast';

	import AddressInput from '$lib/components/forms/AddressInput.svelte';
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import Button from '$lib/components/forms/Button.svelte';
	import CanopyInput from '$lib/components/forms/CanopyInput.svelte';
	import CircumferenceInput from '$lib/components/forms/CircumferenceInput.svelte';
	import Header from '$lib/components/tree/Header.svelte';
	import HeightInput from '$lib/components/forms/HeightInput.svelte';
	import LocationInput from '$lib/components/forms/LocationInput.svelte';
	import NotesInput from '$lib/components/forms/NotesInput.svelte';
	import SpeciesInput from '$lib/components/forms/SpeciesInput.svelte';
	import StateInput from '$lib/components/forms/StateInput.svelte';

	const { data } = $props();
	const treeId = data.id;

	let species = $state(data.tree.species ?? '');
	let height = $state(data.tree.height?.toString() ?? '0');
	let diameter = $state(data.tree.diameter?.toString() ?? '0');
	let circumference = $state(data.tree.circumference?.toString() ?? '0');
	let treeState = $state(data.tree.state ?? '');
	let address = $state('');
	let notes = $state(data.tree.notes ?? '');
	let location = $state([data.tree.lat, data.tree.lon]);

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
				height: parseFloat(height),
				diameter: parseFloat(diameter),
				circumference: parseFloat(circumference),
				state: treeState,
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

<svelte:head>
	<title>Edit tree</title>
</svelte:head>

<Header title="Edit tree" />

<div class="form">
	<AuthWrapper>
		<SpeciesInput bind:value={species} />
		<HeightInput bind:value={height} />
		<CanopyInput bind:value={diameter} />
		<CircumferenceInput bind:value={circumference} />
		<StateInput bind:value={treeState} />
		<LocationInput bind:value={location} />
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
</style>
