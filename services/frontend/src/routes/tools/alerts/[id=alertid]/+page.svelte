<script lang="ts">
	import { untrack } from 'svelte';
	import { page } from '$app/state';
	import { locale as appLocale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { formatDateTime } from '$lib/utils/strings';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { locale } from './lang';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	const id = $derived(page.params.id as string);
	const alert = $derived(pageState.alert);
	const reporter = $derived(alert ? pageState.reporter(alert) : undefined);

	$effect(() => {
		const cleanup = pageState.setup();
		return cleanup;
	});

	$effect(() => {
		if (id) {
			untrack(() => pageState.reload(id));
		}
	});
</script>

<svelte:head>
	<title>{locale.tabTitle(id)} — {appLocale.appTitle()}</title>
</svelte:head>

<AuthWrapper permission="alert:manage">
	<article>
		{#if pageState.isLoading}
			<p aria-busy="true">{locale.loading()}</p>
		{:else if pageState.error}
			<p class="error" role="alert">{locale.error(pageState.error.description)}</p>
		{:else if alert}
			<PageHeader
				text={locale.heading(alert.id)}
				button={{ label: locale.edit(), link: routes.alertEdit(alert.id) }}
			/>

			<dl class="details">
				<dt>{locale.source()}</dt>
				<dd>{alert.source === 'telegram' ? locale.telegram() : locale.web()}</dd>

				<dt>{locale.reporter()}</dt>
				<dd>
					{#if reporter}
						<a href="/tools/users/{reporter.id}">{reporter.name}</a>
					{:else if alert.source === 'telegram' && alert.username}
						<a href={`https://t.me/${alert.username}`} target="_blank" rel="noreferrer"
							>@{alert.username}</a
						>
					{:else}
						{locale.unknownReporter()}
					{/if}
				</dd>

				<dt>{locale.status()}</dt>
				<dd>{alert.status}</dd>

				<dt>{locale.createdAt()}</dt>
				<dd>{formatDateTime(alert.created_at)}</dd>

				{#if alert.responded_at !== null}
					<dt>{locale.respondedAt()}</dt>
					<dd>{formatDateTime(alert.responded_at)}</dd>
				{/if}

				<dt>{locale.coordinates()}</dt>
				<dd>
					{#if alert.lat !== null && alert.lon !== null}
						<span>{alert.lat.toFixed(6)}, {alert.lon.toFixed(6)}</span>
						<span class="coord-links">
							<a
								href={`https://www.openstreetmap.org/?mlat=${alert.lat}&mlon=${alert.lon}#map=19/${alert.lat}/${alert.lon}`}
								target="_blank"
								rel="noreferrer">{locale.osm()}</a
							>
							<a
								href={`https://www.google.com/maps?q=${alert.lat},${alert.lon}&t=h&z=19`}
								target="_blank"
								rel="noreferrer">{locale.google()}</a
							>
						</span>
					{:else}
						{locale.emptyValue()}
					{/if}
				</dd>

				{#if alert.response_text !== null}
					<dt>{locale.response()}</dt>
					<dd class="pre-wrap">{alert.response_text}</dd>
				{/if}
			</dl>

			<section>
				<h2>{locale.description()}</h2>
				<p class="pre-wrap">{alert.description || locale.emptyValue()}</p>
			</section>

			{#if pageState.photos.length > 0}
				<section>
					<h2>{locale.photos()}</h2>
					<div class="photo-grid">
						{#each pageState.photos as photo}
							<a href={photo} target="_blank" rel="noopener noreferrer">
								<img src={photo} alt={locale.photoAlt(alert.id)} loading="lazy" />
							</a>
						{/each}
					</div>
				</section>
			{/if}
		{/if}

		<div class="actions">
			<Button type="secondary" link={routes.toolsAlerts()}>{locale.backToAlerts()}</Button>
		</div>
	</article>
</AuthWrapper>

<style>
	.error {
		color: red;
	}

	.details {
		display: grid;
		grid-template-columns: max-content minmax(0, 1fr);
		gap: 0.75rem 1.25rem;
	}

	.details dt {
		font-weight: 600;
		opacity: 0.7;
	}

	.details dd {
		margin: 0;
		overflow-wrap: anywhere;
	}

	.pre-wrap {
		white-space: pre-wrap;
	}

	.coord-links {
		display: inline-flex;
		gap: 0.75rem;
		margin-left: 0.75rem;
	}

	section {
		margin-top: 1.5rem;
	}

	.photo-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
		gap: var(--gap);
	}

	.photo-grid a {
		display: block;
		aspect-ratio: 1;
		overflow: hidden;
		border-radius: 4px;
		background-color: light-dark(#eee, #333);
	}

	.photo-grid img {
		display: block;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.actions {
		margin-top: 1.5rem;
	}

	@media screen and (max-width: 600px) {
		.details {
			grid-template-columns: 1fr;
			gap: 0.25rem;
		}

		.details dd {
			margin-bottom: 0.75rem;
		}
	}
</style>
