<script lang="ts">
	import TreeListPreview from '$lib/components/TreeListPreview.svelte';
	import Comment from '$lib/components/tree/Comment.svelte';
	import Tabs from '$lib/components/updates/Tabs.svelte';
	import { Header } from '$lib/ui';

	const { data } = $props();
	const { comments } = data;
</script>

<Header title="Recent Comments" />
<Tabs active="comments" />

<div class="padded">
	{#each comments as comment}
		<TreeListPreview id={comment.tree_id} />
		<Comment {comment} />
	{/each}
</div>
