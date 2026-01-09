<script lang="ts">
	import UserList from '$lib/components/admin/UserList/index.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<h1>Users</h1>
{#if data.error}
	<p class="error">Error loading users: {data.error.description}</p>
{:else}
	<UserList users={data.users} />
{/if}

<style>
	.error {
		color: red;
	}
</style>
