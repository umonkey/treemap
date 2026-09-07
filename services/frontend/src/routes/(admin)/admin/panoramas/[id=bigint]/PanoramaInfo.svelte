<script lang="ts">
	import type { Panorama } from '$lib/api/panoramas';
	import { PanoramaInfoLogic } from './PanoramaInfo.svelte.ts';

	const { panorama }: { panorama: Panorama } = $props();
	const componentState = new PanoramaInfoLogic();
</script>

<div class="panorama-details">
	{#if componentState.error}
		<p class="error">Error: {componentState.error.description}</p>
	{/if}
	<table>
		<tbody>
			{#if panorama.failure_reason}
				<tr>
					<th>Failure Reason</th>
					<td class="error">{panorama.failure_reason}</td>
				</tr>
			{/if}

			<tr>
				<th>Images:</th>
				<td>{panorama.image_count}</td>
			</tr>

			<tr>
				<th>Hints:</th>
				<td>
					<div class="hints-cell">
						<span>{panorama.hints_count ?? 0}</span>
						<button
							type="button"
							class="clear-link"
							disabled={componentState.isClearingHints ||
								!panorama.hints_count ||
								panorama.hints_count === 0}
							onclick={() => componentState.clearHints(panorama.id, panorama)}
						>
							{componentState.isClearingHints ? 'Clearing...' : 'Clear'}
						</button>
					</div>
				</td>
			</tr>

			<tr>
				<th>Processing job status</th>
				<td>{panorama.processing_status ?? 'unknown'}</td>
			</tr>

			<tr>
				<th>Processing time:</th>
				<td>{componentState.formatProcessingTime(panorama.processing_time)}</td>
			</tr>

			<tr>
				<th>Total file size:</th>
				<td>
					{componentState.formatFileSizeGb(panorama.file_size)}
				</td>
			</tr>
		</tbody>
	</table>
</div>

<style>
	.error {
		color: red;
	}

	.panorama-details table {
		width: 100%;
		border-collapse: collapse;
		margin-bottom: 2rem;
	}

	.panorama-details th,
	.panorama-details td {
		padding: 0.5rem 1rem;
		border-bottom: 1px solid light-dark(#ddd, #444);
		text-align: left;
		vertical-align: top;
	}

	.panorama-details th {
		width: 250px;
		font-weight: bold;
	}

	.hints-cell {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	button.clear-link {
		background: none;
		border: none;
		padding: 0;
		margin: 0;
		font: inherit;
		font-size: 0.9rem;
		line-height: inherit;
		color: var(--link-color);
		text-decoration: underline;
		cursor: pointer;
	}

	button.clear-link:hover:not(:disabled) {
		text-decoration: none;
	}

	button.clear-link:disabled {
		opacity: 0.4;
		cursor: default;
		text-decoration: none;
	}
</style>
