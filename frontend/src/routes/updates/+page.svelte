<script lang="ts">
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import Tabs from '$lib/components/updates/Tabs.svelte';
	import { Header } from '$lib/ui';
</script>

<Header title="Updates" />
<Tabs active="new" />

<div class="padded">
	<AuthWrapper>
		<p>This page is under construction. Please come back later.</p>
		<p>This page will contain all recently added trees, comments and other updates.</p>
	</AuthWrapper>
</div>
