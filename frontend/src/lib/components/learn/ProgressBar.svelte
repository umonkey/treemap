<script lang="ts">
	const { total, complete } = $props<{
		total: number;
		complete: number;
	}>();

	console.debug(`[progress] total=${total}, complete=${complete}`);

	const width = () => {
		const val = Math.round((complete / total) * 100);
		return `${val}%`;
	};
</script>

<div class="progress">
	<div class="complete" style="width: {width()}"></div>
</div>

<style>
	.progress {
		width: 100%;
		height: 20px;
		background: var(--color-progress-bg);
		border-radius: 10px;
		overflow: hidden;
	}

	.complete {
		height: 100%;
		background: var(--color-progress-fg);
		width: 0;
		transition: width 0.5s ease-in-out;
	}
</style>
