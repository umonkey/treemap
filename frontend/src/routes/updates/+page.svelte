<script>
import AuthWrapper from "$lib/components/auth/AuthWrapper.svelte";
import Header from "$lib/components/tree/Header.svelte";
import Tabs from "$lib/components/updates/Tabs.svelte";
</script>

<Header title="Updates" />
<Tabs active="new" />

<div class="padded">
	<AuthWrapper>
		<p>This page is under construction. Please come back later.</p>
		<p>This page will contain all recently added trees, comments and other updates.</p>
	</AuthWrapper>
</div>
