<script lang="ts">
	import { Button, Buttons, TreeSheet, FilteredChangeList, AuthWrapper } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { stateUpdater } from '$lib/actions';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, error, history, tree, save, close } = stateUpdater(id, 'dead');
</script>

<AuthWrapper>
	<div class="delete-tree">
		{#if $error}
			<p>{$error.description}</p>
		{:else if $loading}
			<p>Checking the tree...</p>
		{:else}
			<p>{locale.deadHeader()}</p>

			<TreeSheet tree={$tree} />

			<p>{locale.deadUploadHint()}</p>

			<Buttons>
				<Button label={locale.deadConfirm()} onClick={save} disabled={$busy} />
				<Button type="cancel" label={locale.editCancel()} onClick={close} />
			</Buttons>
		{/if}

		<FilteredChangeList changes={$history} name="state" />
	</div>
</AuthWrapper>
