<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { locale } from '$lib/locale';
	import Ruler from '$lib/icons/Ruler.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	const id = $derived(page.params.id as string);

	onMount(() => {
		pageState.init(id);
		return pageState.destroy;
	});
</script>

{#if pageState.tree}
	<div class="props">
		<div class="line">
			<div class="icon">
				<Ruler />
			</div>
			<div class="value">
				{locale.distanceLabel(locale.meters(pageState.distance.toFixed(1)))}
			</div>
		</div>
	</div>

	{#if pageState.error}
		<p class="error">{pageState.error}</p>
	{/if}

	<Buttons>
		<Button onClick={() => pageState.handleConfirm(id)} disabled={pageState.busy} nowrap
			>{locale.confirm()}</Button
		>
		<Button type="secondary" onClick={() => pageState.handleCancel(id)} nowrap
			>{locale.editCancel()}</Button
		>
	</Buttons>
{/if}

<style>
	.props {
		opacity: 0.7;
		display: flex;
		flex-direction: column;
		gap: 5px;

		.line {
			white-space: nowrap;
			overflow: hidden;
			text-overflow: ellipsis;

			display: flex;
			flex-direction: row;
			align-items: center;
			gap: var(--gap);

			.icon {
				width: 20px;
				height: 20px;
			}
		}
	}

	.error {
		color: var(--color-error);
		margin: 0;
		font-size: 0.9rem;
	}
</style>
