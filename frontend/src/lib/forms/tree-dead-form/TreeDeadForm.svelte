<script lang="ts">
	import { Button, Buttons, Form, AuthWrapper } from '$lib/ui';
	import TreeSheet from '$lib/components/tree/TreeSheet.svelte';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { locale } from '$lib/locale';
	import { stateUpdater } from '$lib/actions';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, error, tree, save, close } = stateUpdater(id, 'dead');
</script>

<AuthWrapper>
	{#if $error}
		<p>{$error.description}</p>
	{:else if $loading}
		<p>Checking the tree...</p>
	{:else}
		<Title title={$tree?.species} address={$tree?.address} />
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
