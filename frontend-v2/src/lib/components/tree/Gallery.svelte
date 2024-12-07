<script>
	import { routes } from '$lib/routes';

	export let files = [];

	const added_at = () => {
		const date = '07.11.2024';
		const author = 'John Doe';
		return `${date} by ${author}`;
	};
</script>

<div class="gallery">
	<div class="imgno">1/{files.length}</div>
	<div class="slides">
		{#each files as file}
			<div>
				<img src={routes.file(file.small_id)} alt="See how good is this tree." />
				{#if file.added_at}
					<div class="date">{added_at(file)}</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<style>
	.gallery {
		position: relative;
		box-sizing: border-box;
	}

	.imgno {
		position: absolute;
		right: var(--gap);
		top: var(--gap);
		background-color: rgba(0, 0, 0, 0.85);
		color: #fff;
		border-radius: 9px;
		font-size: 14px;
		padding: 2px 8px;
		z-index: 2;
	}

	.slides {
		display: flex;
		max-width: 100%;
		aspect-ratio: 1/1;

		overflow-x: auto;
		overflow-y: hidden;
		white-space: nowrap;
		scroll-snap-type: x mandatory;

		& > div {
			display: block;
			position: relative;

			min-height: 100%;
			min-width: 100%;

			aspect-ratio: 1/1;
			text-align: center;
			align-content: center;
			scroll-snap-align: center;

			img {
				border: none;
				object-position: center;
				object-fit: cover;
				height: 100%;
				width: 100%;
			}
		}

		.date {
			position: absolute;
			bottom: var(--gap);
			left: var(--gap);
			background-color: rgba(0, 0, 0, 0.75);
			color: #fff;
			font-size: 14px;
			padding: 2px 4px;

			/* Prevent text overflow in case of really long username */
			max-width: calc(100% - 2 * var(--gap) - 8px);
			overflow: hidden;
			text-overflow: ellipsis;
		}
	}
</style>
