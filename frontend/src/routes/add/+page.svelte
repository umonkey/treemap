<script lang="ts">
	import { goto } from '$app/navigation';
	import { toast } from '@zerodevx/svelte-toast';
	import { apiClient } from '$lib/api';
	import { routes } from '$lib/routes';
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

	let busy = $state(false);

	let species = $state('');
	let height = $state<number | null>(null);
	let diameter = $state<number | null>(null);
	let circumference = $state<number | null>(null);
	let treeState = $state<string | null>(null);
	let notes = $state('');
	let location = $state([data.lat, data.lng]);
	let year = $state<number | null>(null);

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
				height,
				diameter,
				circumference,
				state: treeState,
				notes,
				year
			})
			.then((res) => {
				if (res.data) {
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
	<title>{locale.addTitle()}</title>
</svelte:head>

<Header title={locale.addTitle()} />

<div class="form">
	<AuthWrapper>
		<LocationInput bind:value={location} label={locale.addConfirmLocation()} />
		<SpeciesInput bind:value={species} />
		<HeightInput value={height} onChange={(value: number | null) => (height = value)} />
		<CanopyInput value={diameter} onChange={(value: number | null) => (diameter = value)} />
		<CircumferenceInput
			value={circumference}
			onChange={(value: number) => (circumference = value)}
		/>
		<StateInput value={treeState} onChange={(value: string | null) => (treeState = value)} />
		<YearInput value={year} onChange={(value: number) => (year = value)} />
		<NotesInput bind:value={notes} />

		<div class="buttons">
			<Button type="submit" label={locale.addConfirmButton()} onClick={onSave} disabled={busy} />
			<Button type="cancel" label={locale.addCancelButton()} onClick={onCancel} disabled={busy} />
		</div>
	</AuthWrapper>
</div>

<style>
	.form {
		padding: 0 var(--gap) var(--gap);
	}
</style>
