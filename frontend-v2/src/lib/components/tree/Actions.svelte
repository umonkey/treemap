<script lang="ts">
	import { routes } from '$lib/routes';

	import ChatIcon from '$lib/icons/ChatIcon.svelte';
	import HeartIcon from '$lib/icons/HeartIcon.svelte';
	import SaveIcon from '$lib/icons/SaveIcon.svelte';
	import ShareButton from '$lib/components/tree/ShareButton.svelte';

	const { tree } = $props();
</script>

<div class="actions">
	<!--
	<div><HeartIcon class="icon" /></div>
	-->
	<div><a href={routes.treeComments(tree)}><ChatIcon class="icon" /></a></div>
	<div><ShareButton /></div>
	<!--
	<div class="sep"></div>
	<div><SaveIcon class="icon" /></div>
	-->
</div>

<style>
	.actions {
		display: flex;
		gap: calc(var(--gap) * 1.5);
		justify-content: left;
		margin: var(--gap) 0;
		padding: 0 var(--gap);

		& > div {
			width: 20px;
			height: 20px;
			color: var(--icon-color-secondary);
			text-align: center;
		}

		.sep {
			flex-grow: 1;
		}

		a {
			color: inherit;
		}
	}
</style>
