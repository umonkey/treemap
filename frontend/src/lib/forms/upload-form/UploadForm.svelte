<script lang="ts">
	import { routes } from '$lib/routes';
	import { fileAttribution } from '$lib/utils/strings';
	import { hooks } from './hooks';
	import { Buttons, Button } from '$lib/ui';

	const { id } = $props<{ id: string }>();
	const { loading, tree, thumbnail, error, reload, handleMakeThumbnail, handleDelete } = hooks();

	$effect(() => reload(id));
</script>

{#if $loading}
	<!-- loading -->
{:else if $error}
	<p>{$error}</p>
{:else if $tree?.files?.length > 0}
	<h2>Manage existing photos</h2>

	<div class="pics">
		{#each $tree.files as file}
			<div class="pic">
				<a href={routes.file(file.id)} class="thumbnail" target="_blank">
					<img src={routes.file(file.small_id)} alt="thumbnail" />
				</a>
				<div class="props">
					<div class="by">{fileAttribution(file)}</div>

					<Buttons>
						<Button
							disabled={file.small_id === $thumbnail}
							onClick={() => handleMakeThumbnail(file)}
							label="Make thumbnail"
						/>

						<Button
							disabled={file.small_id === $thumbnail}
							onClick={() => handleDelete(file.id)}
							label="Delete"
						/>
					</Buttons>
				</div>
			</div>
		{/each}
	</div>
{/if}

<style>
	.pics {
		display: flex;
		flex-direction: column;
		gap: var(--gap);
		width: 100%;
	}

	.pic {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
	}

	.thumbnail {
		display: block;
		position: relative;
		line-height: 0;
		flex-basis: 100px;
		flex-grow: 0;
		flex-shrink: 0;

		img {
			width: 100%;
			aspect-ratio: 1;
			object-position: center;
			object-fit: cover;
		}
	}

	.props {
		display: flex;
		flex-direction: column;
		gap: var(--gap);
	}
</style>
