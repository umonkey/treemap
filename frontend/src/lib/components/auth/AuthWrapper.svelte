<script>
	import { isAuthenticated } from '$lib/stores/auth';
	import SignIn from '$lib/components/auth/SignIn.svelte';

	let { children } = $props();
</script>

{#if $isAuthenticated}
	{@render children()}
{:else}
	<p>You need to sign in to access this page.</p>
	<SignIn />
{/if}
