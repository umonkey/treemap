<script lang="ts">
	import { LazyImage } from '$lib/ui';
	import { CircumferenceIcon, SpinnerIcon } from '$lib/icons';
	import FALLBACK from '$lib/assets/tree.jpg';

	const { items, onRetry, small } = $props<{
		items: {
			src: string;
			busy: boolean;
			error: boolean;
		}[];
		onRetry: (index: number) => void;
		small?: boolean;
	}>();
</script>

<div class="items" class:small={!!small}>
	{#each items as item, idx}
		<div class="item preview" class:uploading={item.busy} class:error={item.error}>
			<div class="img">
				<LazyImage src={item.src} fallback={FALLBACK} alt="preview" />
			</div>
			{#if item.busy}
				<div class="spinner">
					<SpinnerIcon />
				</div>
			{/if}
			{#if item.error}
				<button class="retry" onclick={() => onRetry(idx)}>
					<CircumferenceIcon />
				</button>
			{/if}
		</div>
	{/each}
</div>

<style>
	.items {
		display: flex;
		flex-direction: row;
		gap: 10px;

		width: 100%;
		overflow-x: auto;

		height: 100px;

		/* Testing mobile UI in Storybook */
		&.small {
			height: 50px;

			.item {
				flex-basis: 50px;
			}
		}
	}

	.item {
		flex-shrink: 0;
		flex-grow: 0;
		flex-basis: 100px;

		/* Allows to position the retry icon */
		position: relative;

		aspect-ratio: 1;

		aspect-ratio: 1;
		box-sizing: border-box;
		border-radius: 8px;
		overflow: hidden;

		&.uploading .img {
			opacity: 0.2;
		}

		&.error .img {
			filter: grayscale(100%);
			opacity: 0.2;
		}
	}

	.spinner,
	.retry {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		cursor: pointer;

		box-sizing: border-box;
		padding: 10px;
	}

	.retry {
		background-color: transparent;
		outline: none;
		border: none;
		color: #fff;
	}

	@media (max-width: 1023px) {
		.items {
			height: 50px;

			.item {
				flex-basis: 50px;
			}
		}
	}
</style>
