<script lang="ts">
	import UserDetails from '$lib/components/admin/UserDetails/index.svelte';
	import NarrowPage from '$lib/ui/narrow-page/NarrowPage.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<NarrowPage>
	<UserDetails user={data.user} />
</NarrowPage>
