<script lang="ts">
	import type { Snippet } from 'svelte';
	import { isAuthenticated, hasPermission } from '$lib/stores/authStore';
	import SignInButton from '../sign-in-button/SignInButton.svelte';

	interface Props {
		children: Snippet;
		permission?: string | string[];
	}

	const { children, permission }: Props = $props();

	const hasAccess = $derived.by(() => {
		if (!permission) return true;
		if (Array.isArray(permission)) {
			return permission.some((p) => $hasPermission(p));
		}
		return $hasPermission(permission);
	});
</script>

{#if $isAuthenticated}
	{#if hasAccess}
		{@render children()}
	{:else}
		<p>You don't have permission to access this page.</p>
	{/if}
{:else}
	<p>You need to sign in to access this page.</p>
	<SignInButton />
{/if}
