<script lang="ts">
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { CircumferenceInput } from '$lib/ui';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, loadError, saveError, busy, tree, value, save, close, handleChange } =
		editor(id);
</script>

{#if $loading}
	<!-- loading -->
{:else if $loadError}
	<p>{$loadError}</p>
{:else if $tree}
	<TreeForm {id} title="Tree Circumference" onSubmit={save} onCancel={close} saving={$busy}>
		<CircumferenceInput value={$value} autofocus onChange={handleChange} />

		{#if $saveError}
			<p>{$saveError}</p>
		{/if}

		<ChangeHistory {id} name="circumference" />
	</TreeForm>
{/if}
