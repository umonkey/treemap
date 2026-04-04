<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import type { PageData } from './$types';
	import UserDetails from './UserDetails.svelte';

	let { data }: { data: PageData } = $props();
</script>

<Dialog title="User Properties">
	<UserDetails user={data.user} />
</Dialog>
