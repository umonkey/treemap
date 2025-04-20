<script lang="ts">
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import { HeightEditor } from '$lib/ui';
	import { loadTree } from '$lib/hooks';
	import { routes, goto } from '$lib/routes';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, error, data, reload } = loadTree();

	$effect(() => {
		(async () => await reload(id))();
	});

	const handleClose = () => {
		goto(routes.treeDetails(id));
	};
</script>

<div class="padded measure">
	<AuthWrapper>
		{#if $loading}
			<p>Checking the tree...</p>
		{:else if $error}
			<p>{$error.description}</p>
		{:else}
			<HeightEditor tree={$data} onClose={handleClose} />
		{/if}
	</AuthWrapper>
</div>
