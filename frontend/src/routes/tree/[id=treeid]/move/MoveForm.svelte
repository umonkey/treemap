<script lang="ts">
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import LocationInput from '$lib/ui/location-input/LocationInput.svelte';
	import ChangeHistory from '../components/ChangeHistory.svelte';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, loadError, saveError, tree, value, save, close, handleChange } =
		editor(id);
</script>

{#if $loading}
	<!-- loading -->
{:else if $loadError}
	<p>{$loadError}</p>
{:else if $tree}
	<TreeForm {id} title="Move Tree" onSubmit={save} onCancel={close} disabled={$busy}>
		<LocationInput value={$value} onChange={handleChange} open />

		{#if $saveError}
			<p>{$saveError}</p>
		{/if}

		<ChangeHistory {id} name="location" />
	</TreeForm>
{/if}
