<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import UserList from '$lib/components/admin/UserList/index.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<Dialog title="User List">
	{#if data.error}
		<p class="error">Error loading users: {data.error.description}</p>
	{:else}
		<UserList users={data.users} />
	{/if}
</Dialog>

<style>
	.error {
		color: red;
	}
</style>
