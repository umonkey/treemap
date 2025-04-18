<script lang="ts">
	import { goto } from '$app/navigation';
	import { apiClient } from '$lib/api';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { toast } from '@zerodevx/svelte-toast';

	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import {
		Button,
		CanopyInput,
		CircumferenceInput,
		Header,
		HeightInput,
		LocationInput,
		NotesInput,
		StateInput,
		YearInput,
		SpeciesInput
	} from '$lib/ui';

	const { data } = $props();
	const treeId = data.id;

	let species = $state<string | null>(data.tree.species);
	let height = $state<number | null>(data.tree.height);
	let diameter = $state<number | null>(data.tree.diameter);
	let circumference = $state<number | null>(data.tree.circumference);
	let treeState = $state<string>(data.tree.state ?? 'unknown');
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

	const handleSpeciesChange = (value: string) => {
		species = value;
	};

	const handleHeightChange = (value: number | null) => {
		height = value;
	};

	const handleCanopyChange = (value: number | null) => {
		diameter = value;
	};

	const handleCircumferenceChange = (value: number | null) => {
		circumference = value;
	};

	const handleStateChange = (value: string) => {
		treeState = value;
	};

	const handleYearChange = (value: number | null) => {
		year = value;
	};

	const handleNotesChange = (value: string) => {
		notes = value;
	};

	const handleLocationChange = (value) => {
		location = value;
	};
</script>

<svelte:head>
	<title>{locale.editTitle()}</title>
</svelte:head>

<Header title={locale.editTitle()} />

<div class="form">
	<AuthWrapper>
		<SpeciesInput value={species} onChange={handleSpeciesChange} />
		<HeightInput value={height} onChange={handleHeightChange} />
		<CanopyInput value={diameter} onChange={handleCanopyChange} />
		<CircumferenceInput value={circumference} onChange={handleCircumferenceChange} />
		<StateInput value={treeState} onChange={handleStateChange} />
		<YearInput value={year} onChange={handleYearChange} />
		<LocationInput value={location} onChange={handleLocationChange} />
		<NotesInput value={notes} onChange={handleNotesChange} />

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
