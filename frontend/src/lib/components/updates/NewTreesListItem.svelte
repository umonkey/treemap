<script lang="ts">
import { locale } from "$lib/locale";
import { routes } from "$lib/routes";
import { getUser } from "$lib/stores/userStore";
import { formatDate } from "$lib/utils/strings";
import { formatSpecies } from "$lib/utils/trees";
import { get } from "svelte/store";

import LazyTreeThumbnail from "$lib/components/LazyTreeThumbnail.svelte";

const { tree, extra = undefined } = $props();

const user = get(getUser)(tree.added_by);
const date = formatDate(tree.added_at);
</script>

<div class="tree">
	<a href={routes.treeDetails(tree.id)} class="thumbnail">
		<LazyTreeThumbnail {tree} />
	</a>

	<div class="details">
		<div class="species">
			<a href={routes.treeDetails(tree.id)}>{formatSpecies(tree.species)}</a>
		</div>
		<div class="address">
			{#if tree.address}{tree.address}{:else}{locale.addressUnknown()}{/if}
		</div>
		<div class="added">
			{date} &middot; {#if user}{user.name}{:else}{locale.userUnknown()}{/if}
		</div>
	</div>

	<div class="extra">
		{#if extra}
			{extra}
		{/if}
	</div>
</div>

<style>
	.tree {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
		margin-bottom: var(--gap);
	}

	.thumbnail {
		flex-basis: 75px;
		flex-shrink: 0;
		flex-grow: 0;
	}

	.details {
		line-height: 150%;
		flex-grow: 1;
	}
</style>
