<script lang="ts">
	import { apiClient } from '$lib/api';
	import { goto } from '$app/navigation';
	import { routes } from '$lib/routes';
	import { toast } from '@zerodevx/svelte-toast';
	import { locale } from '$lib/locale';

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
	const treeId = data.id;

	let species = $state<string | null>(data.tree.species);
	let height = $state<number | null>(data.tree.height);
	let diameter = $state<number | null>(data.tree.diameter);
	let circumference = $state<number | null>(data.tree.circumference);
	let treeState = $state<string | null>(data.tree.state);
	let notes = $state(data.tree.notes ?? '');
	let location = $state([data.tree.lat, data.tree.lon]);
	let year = $state<number | null>(data.tree.year);

	const onSave = () => {
		apiClient
			.updateTree(treeId, {
				species,
				height,
				diameter,
				circumference,
				state: treeState,
				notes,
				lat: location[0],
				lon: location[1],
				year
			})
			.then((res) => {
				if (res.status >= 200 && res.status < 400) {
					goto(routes.treeDetails(treeId));
					toast.push('Tree updated.');
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
	<title>{locale.editTitle()}</title>
</svelte:head>

<Header title={locale.editTitle()} />

<div class="form">
	<AuthWrapper>
		<SpeciesInput bind:value={species} />
		<HeightInput value={height} onChange={(value: number | null) => (height = value)} />
		<CanopyInput value={diameter} onChange={(value: number | null) => (diameter = value)} />
		<CircumferenceInput
			value={circumference}
			onChange={(value: number) => (circumference = value)}
		/>
		<StateInput value={treeState} onChange={(value: string | null) => (treeState = value)} />
		<YearInput value={year} onChange={(value: number | null) => (year = value)} />
		<LocationInput bind:value={location} />
		<NotesInput bind:value={notes} />

		<div class="buttons">
			<Button type="submit" label={locale.editSave()} onClick={onSave} />
			<Button type="cancel" label={locale.editCancel()} onClick={onCancel} />
		</div>
	</AuthWrapper>
</div>

<style>
	.form {
		padding: 0 var(--gap) var(--gap);
	}
</style>
