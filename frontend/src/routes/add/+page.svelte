<script lang="ts">
	import { goto } from '$app/navigation';
	import { toast } from '@zerodevx/svelte-toast';
	import { apiClient } from '$lib/api';
	import { routes } from '$lib/routes';

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
	import YearInput from '$lib/components/forms/YearInput.svelte';

	const { data } = $props();

	let busy = $state(false);

	let species = $state('');
	let height = $state('');
	let diameter = $state('');
	let circumference = $state('');
	let treeState = $state('');
	let notes = $state('');
	let location = $state([data.lat, data.lng]);
	let year = $state('');

	const numeric = (value: string): number | null => {
		if (value === '') {
			return null;
		}

		if (value === '0') {
			return null;
		}

		const num = parseFloat(value);
		return isNaN(num) ? null : num;
	};

	const onSave = () => {
		busy = true;

		apiClient
			.addTree({
				points: [
					{
						lat: location[0],
						lon: location[1]
					}
				],
				species,
				height: numeric(height),
				diameter: numeric(diameter),
				circumference: numeric(circumference),
				state: treeState,
				notes,
				year: numeric(year)
			})
			.then((res) => {
				if (res.status >= 200 && res.status < 400) {
					goto(routes.treeDetails(res.data.trees[0].id));
				} else {
					console.error(`Error ${res.status} adding tree.`);
					toast.push('Error adding tree.');
				}
			})
			.catch((e) => {
				console.error(`Error adding tree: ${e}.`);
				toast.push('Error adding tree.');
			})
			.finally(() => {
				busy = false;
			});
	};

	const onCancel = () => {
		history.back();
	};
</script>

<svelte:head>
	<title>Add tree</title>
</svelte:head>

<Header title="Add Tree" />

<div class="form">
	<AuthWrapper>
		<LocationInput bind:value={location} label="Confirm location" />
		<SpeciesInput bind:value={species} />
		<HeightInput bind:value={height} />
		<CanopyInput bind:value={diameter} />
		<CircumferenceInput bind:value={circumference} />
		<StateInput bind:value={treeState} />
		<YearInput bind:value={year} />
		<NotesInput bind:value={notes} />

		<div class="buttons">
			<Button type="submit" label="Add tree" onClick={onSave} disabled={busy} />
			<Button type="cancel" label="Cancel" onClick={onCancel} disabled={busy} />
		</div>
	</AuthWrapper>
</div>

<style>
	.form {
		padding: 0 var(--gap) var(--gap);
	}
</style>
