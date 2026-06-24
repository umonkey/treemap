<script lang="ts">
	import Layout from '$lib/components/layout/Layout.svelte';
	import { onMount } from 'svelte';

	const { children } = $props();

	onMount(() => {
		document.body.classList.add('fixed');
		return () => {
			document.body.classList.remove('fixed');
		};
	});
</script>

<Layout>
	{@render children()}
</Layout>
