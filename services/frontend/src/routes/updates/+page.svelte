<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import Tabs from '$lib/components/updates/Tabs.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
</script>

<Dialog title="Updates">
	<Tabs active="new" />

	<AuthWrapper>
		<p>This page is under construction. Please come back later.</p>
		<p>This page will contain all recently added trees, comments and other updates.</p>
	</AuthWrapper>
</Dialog>
