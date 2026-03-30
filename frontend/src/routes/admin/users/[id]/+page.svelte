<script lang="ts">
	import UserDetails from '$lib/components/admin/UserDetails/index.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<Dialog title="User Properties">
	<UserDetails user={data.user} />
</Dialog>
