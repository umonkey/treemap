<script lang="ts">
	import { Header, NarrowPage } from '$lib/ui';
	import UserList from '$lib/components/admin/UserList/index.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>User List</title>
</svelte:head>

<Header title="UserList" />

<NarrowPage>
	{#if data.error}
		<p class="error">Error loading users: {data.error.description}</p>
	{:else}
		<UserList users={data.users} />
	{/if}
</NarrowPage>

<style>
	.error {
		color: red;
	}
</style>
