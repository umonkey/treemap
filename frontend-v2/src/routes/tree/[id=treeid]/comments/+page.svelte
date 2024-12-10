<script>
	import Header from '$lib/components/tree/Header.svelte';
	import Tabs from '$lib/components/tree/Tabs.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeMenu from '$lib/components/tree/Menu.svelte';
	import CommentForm from '$lib/components/forms/CommentForm.svelte';

	const { data } = $props();
	const tree = data.tree;
</script>

<svelte:head>
	<title>{tree.species} - Details</title>
</svelte:head>

<Header />
<Title title={tree.species} />
<Tabs tree={tree.id} active="comments" />
<TreeMenu id={tree.id} />

<div class="container">
	<p>No comments for this tree yet.</p>
	<CommentForm />
</div>

<style>
	.container {
		padding: 0 var(--gap);
	}
</style>
