<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import type { PageData } from './$types';
	import UserList from './UserList.svelte';

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
