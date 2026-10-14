<script lang="ts">
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';

	const { children } = $props();
</script>

<AuthWrapper permission="species:manage">
	{@render children()}
</AuthWrapper>
