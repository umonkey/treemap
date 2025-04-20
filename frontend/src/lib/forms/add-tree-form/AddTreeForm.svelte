<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

	import { locale } from '$lib/locale';
	import { apiClient } from '$lib/api';
	import type { ITree, IAddTreeRequest } from '$lib/types';
	import { toast } from '@zerodevx/svelte-toast';

	import {
		Button,
		CanopyInput,
		CircumferenceInput,
		HeightInput,
		LocationInput,
		NotesInput,
		SpeciesInput,
		StateInput,
		YearInput
	} from '$lib/ui';

	const { lat, lng, onAdded, onCancel, onBeforeSubmit } = $props<{
		lat: number | null;
		lng: number | null;
		onAdded: (tree: ITree) => void;
		onCancel: () => void;
		onBeforeSubmit?: (req: IAddTreeRequest) => void;
	}>();

	let busy = $state(false);

	let species = $state('');
	let height = $state<number | null>(null);
	let diameter = $state<number | null>(null);
	let circumference = $state<number | null>(null);
	let treeState = $state<string>('unknown');
	let notes = $state('');
	let location = $state([lat, lng]);
	let year = $state<number | null>(null);

	const onSave = () => {
		busy = true;

		const req = {
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
		} as IAddTreeRequest;

		if (onBeforeSubmit) {
			onBeforeSubmit(req);
		}

		apiClient
			.addTree(req)
			.then((res) => {
				if (res.data) {
					onAdded(res.data.trees[0]);
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

	const handleSpeciesChange = (value: string) => {
		species = value;
	};

	const handleNotesChange = (value: string) => {
		notes = value;
	};

	const handleYearChange = (value: number | null) => {
		year = value;
	};

	const handleStateChange = (value: string) => {
		treeState = value;
	};

	const handleCircumferenceChange = (value: number | null) => {
		circumference = value;
	};

	const handleLocationChange = (value: number[]) => {
		location = value;
	};
</script>

<LocationInput value={location} label={locale.addConfirmLocation()} onChange={handleLocationChange} />
<SpeciesInput value={species} onChange={handleSpeciesChange} />
<HeightInput value={height} onChange={(value: number | null) => (height = value)} />
<CanopyInput value={diameter} onChange={(value: number | null) => (diameter = value)} />
<CircumferenceInput value={circumference} onChange={handleCircumferenceChange} />
<StateInput value={treeState} onChange={handleStateChange} />
<YearInput value={year} onChange={handleYearChange} />
<NotesInput value={notes} onChange={handleNotesChange} />

<div class="buttons">
	<Button type="submit" label={locale.addConfirmButton()} onClick={onSave} disabled={busy} />
	<Button type="cancel" label={locale.addCancelButton()} onClick={onCancel} disabled={busy} />
</div>
