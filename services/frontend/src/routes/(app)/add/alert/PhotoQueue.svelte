<script lang="ts">
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import { locale } from './lang';

	const { items, onRemove }: { items: { url: string }[]; onRemove: (index: number) => void } =
		$props();
</script>

{#if items.length > 0}
	<div class="queue">
		{#each items as item, index}
			<div class="tile">
				<img src={item.url} alt={locale.photoAlt()} />

				<button
					type="button"
					class="remove"
					title={locale.removePhoto()}
					aria-label={locale.removePhoto()}
					onclick={() => onRemove(index)}
				>
					<CloseIcon />
				</button>
			</div>
		{/each}
	</div>
{/if}

<style>
	.queue {
		display: flex;
		flex-direction: row;
		gap: 10px;
		width: 100%;
		overflow-x: auto;
		scrollbar-width: none;
		height: 100px;
	}

	.tile {
		display: block;
		position: relative;
		flex-shrink: 0;
		flex-grow: 0;
		height: 100%;
		aspect-ratio: 1;
		box-sizing: border-box;
		border-radius: 8px;
		overflow: hidden;
		border: 1px solid rgba(128, 128, 128, 0.5);
		background-color: var(--color-dialog-header);
	}

	img {
		display: block;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.remove {
		position: absolute;
		top: 4px;
		right: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 22px;
		height: 22px;
		padding: 3px;
		box-sizing: border-box;
		border: none;
		border-radius: 50%;
		background-color: rgba(0, 0, 0, 0.6);
		color: #fff;
		cursor: pointer;

		:global(svg) {
			display: block;
			width: 14px;
			height: 14px;
		}
	}

	@media (max-width: 1023px) {
		.queue {
			height: 50px;
		}
	}
</style>
