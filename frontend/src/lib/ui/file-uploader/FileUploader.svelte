<script lang="ts">
	import { CameraIcon } from '$lib/icons';
	import { LazyImage } from '$lib/ui';
	import { load } from './hooks';
	import FALLBACK from '$lib/assets/tree.jpg';

	const { input, files, handleChange } = load();
</script>

<div class="uploader">
	<label class="item button">
		<CameraIcon />

		<input
			type="file"
			accept="image/jpeg"
			bind:this={$input}
			onchange={handleChange}
			capture="environment"
		/>
	</label>

	{#each $files as file}
		<div class="item preview">
			<LazyImage src={URL.createObjectURL(file)} fallback={FALLBACK} alt="preview" />
		</div>
	{/each}

	<div class="filler"></div>
</div>

<style>
	.uploader {
		display: flex;
		flex-direction: row;
		gap: 10px;
		justify-content: normal;
	}

	.item {
		flex-shrink: 0;
		flex-grow: 0;
		flex-basis: 100px;
		aspect-ratio: 1;
		box-sizing: border-box;
		border-radius: 8px;
		overflow: hidden;
	}

	.button {
		background-color: #444;
		padding: 10px;
		cursor: pointer;
	}

	.filler {
		flex-shrink: 1;
		flex-grow: 1;
	}

	input {
		visibility: hidden;
		width: 0;
		height: 0;
	}

	@media (max-width: 1023px) {
		.item {
			flex-basis: 50px;
		}
	}
</style>
