<script lang="ts">
	import StreetViewIcon from '$lib/icons/StreetViewIcon.svelte';
	import { formatDate } from '$lib/utils/strings';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import { componentState } from './PanoramaHeader.svelte.ts';

	let {
		id,
		title,
		createdAt,
		status,
		visible = $bindable(false)
	}: {
		id: string;
		title: string;
		createdAt: number;
		status: string;
		visible: boolean;
	} = $props();
</script>

<div class="panorama-header">
	<div class="left-cell">
		<div class="icon-wrapper">
			<StreetViewIcon />
		</div>
	</div>
	<div class="middle-cell">
		<div class="title">{title}</div>
		<div class="subtitle">{formatDate(createdAt)} - {status}</div>
	</div>
	<div class="right-cell">
		<CheckInput
			bind:value={visible}
			onChange={(v) => componentState.updateVisibility(id, v)}
			disabled={status != 'SUCCESS'}
		/>
	</div>
</div>

<style>
	.panorama-header {
		display: flex;
		align-items: center;
		background: var(--form-background);
		border-radius: 12px;
		padding: 0.75rem 1rem;
		gap: 1rem;
	}

	.left-cell {
		width: 48px;
		height: 48px;
		min-width: 48px;
		min-height: 48px;
		background: var(--background-color);
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.icon-wrapper {
		width: 24px;
		height: 24px;
	}

	.middle-cell {
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		min-width: 0;
	}

	.title {
		font-weight: bold;
		font-size: 1.1rem;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.subtitle {
		font-size: 0.85rem;
		color: var(--pico-muted-color, var(--secondary-color, #707070));
	}

	.right-cell {
		display: flex;
		align-items: center;
		padding-left: 1rem;
		font-weight: bold;
		font-size: 1rem;
		min-width: 3rem;
		justify-content: center;
	}
</style>
