<script lang="ts">
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import { Button, Buttons, TreeSheet } from '$lib/ui';
	import { apiClient } from '$lib/api';
	import { loadTree } from '$lib/hooks';
	import { locale } from '$lib/locale';
	import { routes, goto } from '$lib/routes';
	import { toast } from '@zerodevx/svelte-toast';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, error, data, reload } = loadTree();
	let busy = $state<boolean>(false);

	$effect(() => {
		(async () => await reload(id))();
	});

	const handleConfirm = async () => {
		try {
			busy = true;
			await apiClient.updateTreeState(id, 'gone');
			toast.push(locale.deleteNotification());
			goto(routes.treeDetails(id));
		} catch (e) {
			console.error(`Error deleting tree: ${e}`);
			toast.push('Error deleting tree.');
		} finally {
			busy = false;
		}
	};

	const handleCancel = () => {
		goto(routes.treeDetails(id));
	};
</script>

<AuthWrapper>
	<div class="delete-tree">
		{#if $error}
			<p>{$error.description}</p>
		{:else if $loading}
			<p>Checking the tree...</p>
		{:else}
			<p>{locale.deleteHeader()}</p>

			<TreeSheet tree={$data} />

			<p>{locale.deleteUploadHint()}</p>

			<Buttons>
				<Button label={locale.deleteConfirm()} onClick={handleConfirm} disabled={busy} />
				<Button type="cancel" label={locale.editCancel()} onClick={handleCancel} />
			</Buttons>
		{/if}
	</div>
</AuthWrapper>
