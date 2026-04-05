<script lang="ts">
	import { stateUpdater } from '$lib/actions/state-update/hooks';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { locale } from '$lib/locale';
	import ChangeHistory from '../components/ChangeHistory.svelte';
	import TreeSheet from '../components/TreeSheet.svelte';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, error, tree, save, close } = stateUpdater(id, 'dead');
</script>

{#if $error}
	<p>{$error}</p>
{:else if $loading}
	<p>Checking the tree...</p>
{:else if $tree}
	<TreeForm {id} title="Dead Tree" onSubmit={save} onCancel={close} disabled={$busy}>
		<p>{locale.deadHeader()}</p>

		<TreeSheet tree={$tree} />

		<p>{locale.deadUploadHint()}</p>

		<ChangeHistory {id} name="state" />
	</TreeForm>
{/if}

<style>
	p {
		margin: 0;
	}
</style>
