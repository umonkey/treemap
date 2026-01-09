<script lang="ts">
	import UserDetails from '$lib/components/admin/UserDetails/index.svelte';
	import { Header, NarrowPage } from '$lib/ui';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>User Properties</title>
</svelte:head>

<Header title="User Properties" />

<NarrowPage>
	<UserDetails user={data.user} />
</NarrowPage>
