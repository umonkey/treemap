<script>
import SignIn from "$lib/components/auth/SignIn.svelte";
import { isAuthenticated } from "$lib/stores/authStore";

const { children } = $props();
</script>

{#if $isAuthenticated}
	{@render children()}
{:else}
	<p>You need to sign in to access this page.</p>
	<SignIn />
{/if}
