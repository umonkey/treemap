<script lang="ts">
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import { Button, Buttons, TreeSheet } from '$lib/ui';
	import { loadTree } from '$lib/hooks';
	import { locale } from '$lib/locale';
	import { markDead } from '$lib/actions';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, error, data, reload } = loadTree();
	const { busy, handleConfirm, handleCancel } = markDead(id);

	$effect(() => {
		(async () => await reload(id))();
	});
</script>

<AuthWrapper>
	<div class="delete-tree">
		{#if $error}
			<p>{$error.description}</p>
		{:else if $loading}
			<p>Checking the tree...</p>
		{:else}
			<p>{locale.deadHeader()}</p>

			<TreeSheet tree={$data} />

			<p>{locale.deadUploadHint()}</p>

			<Buttons>
				<Button label={locale.deadConfirm()} onClick={handleConfirm} disabled={$busy} />
				<Button type="cancel" label={locale.editCancel()} onClick={handleCancel} />
			</Buttons>
		{/if}
	</div>
</AuthWrapper>
