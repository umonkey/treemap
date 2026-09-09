<script lang="ts">
	import { page } from '$app/stores';
	import { locale } from '$lib/locale';
	import { goto, routes } from '$lib/routes';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import LocationIcon from '$lib/icons/LocationIcon.svelte';
	import TagIcon from '$lib/icons/TagIcon.svelte';
	import { formatDate } from '$lib/utils/strings';
	import { pageState } from './hooks.svelte.ts';

	const id = $derived($page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});

	const title = $derived(
		pageState.source
			? `${locale.waterTitle()} — ${locale.appTitle()}`
			: `${locale.waterTitle()} — ${locale.appTitle()}`
	);
</script>

<svelte:head>
	<title>{title}</title>
</svelte:head>

<Dialog
	title={locale.waterTitle()}
	onCancel={() => goto(routes.map())}
	buttons={[
		{
			title: locale.waterMoveButton(),
			onClick: () => goto(routes.waterMove(id))
		}
	]}
>
	{#if pageState.source}
		<div class="props">
			<div class="line">
				<div class="icon">
					<TagIcon />
				</div>
				<div class="value">{locale.waterStatus(pageState.source.status)}</div>
			</div>
			<div class="line">
				<div class="icon">
					<LocationIcon />
				</div>
				<div class="value">
					{pageState.source.lat.toFixed(6)}, {pageState.source.lon.toFixed(6)}
				</div>
			</div>
			<div class="line">
				<div class="icon">
					<LocationIcon />
				</div>
				<div class="value">{locale.waterAddedAt(formatDate(pageState.source.created_at))}</div>
			</div>
		</div>
	{:else}
		<p>Loading...</p>
	{/if}
</Dialog>

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
</style>
