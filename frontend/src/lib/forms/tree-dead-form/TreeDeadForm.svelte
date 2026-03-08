<script lang="ts">
	import { stateUpdater } from '$lib/actions';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import TreeSheet from '$lib/components/tree/TreeSheet.svelte';
	import { locale } from '$lib/locale';
	import { AuthWrapper, Button, Buttons, Form } from '$lib/ui';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, error, tree, save, close } = stateUpdater(id, 'dead');
</script>

<AuthWrapper>
	{#if $error}
		<p>{$error}</p>
	{:else if $loading}
		<p>Checking the tree...</p>
	{:else if $tree}
		<Title title={$tree.species} address={$tree.address ?? undefined} />
		<TreeContextMenu id={$tree.id} />

		<Form onSubmit={save} sticky>
			<p>{locale.deadHeader()}</p>

			<TreeSheet tree={$tree} />

			<p>{locale.deadUploadHint()}</p>

			<Buttons sticky>
				<Button onClick={save} disabled={$busy}>{locale.deadConfirm()}</Button>
				<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
			</Buttons>

			<ChangeHistory {id} name="state" />
		</Form>
	{/if}
</AuthWrapper>

<style>
	p {
		margin: 0;
	}
</style>
