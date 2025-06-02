<script lang="ts">
	import { Button, Buttons, TreeSheet, FilteredChangeList, AuthWrapper } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { stateUpdater } from '$lib/actions';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, error, history, tree, save, close } = stateUpdater(id, 'gone');
</script>

<AuthWrapper>
	<div class="delete-tree">
		{#if $error}
			<p>{$error.description}</p>
		{:else if $loading}
			<p>Checking the tree...</p>
		{:else}
			<p>{locale.deleteHeader()}</p>

			<TreeSheet tree={$tree} />

			<p>{locale.deleteUploadHint()}</p>

			<Buttons>
				<Button onClick={save} disabled={$busy}>{locale.deleteConfirm()}</Button>
				<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
			</Buttons>

			<FilteredChangeList changes={$history} name="state" />
		{/if}
	</div>
</AuthWrapper>
