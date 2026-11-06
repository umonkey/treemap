<script lang="ts">
	import { untrack } from 'svelte';
	import { locale as appLocale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { formatDate } from '$lib/utils/strings';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { locale } from './lang';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	$effect(() => {
		const cleanup = pageState.setup();
		untrack(() => pageState.reload());
		return cleanup;
	});
</script>

<svelte:head>
	<title>{locale.title()} — {appLocale.appTitle()}</title>
</svelte:head>

<AuthWrapper permission="alert:manage">
	<article>
		<PageHeader text={locale.heading()} />

		<p class="notice">{locale.notice()}</p>

		{#if pageState.isLoading}
			<p aria-busy="true">{locale.loading()}</p>
		{:else if pageState.error}
			<p class="error" role="alert">{locale.error(pageState.error.description)}</p>
		{:else if pageState.alerts.length > 0}
			<div class="alert-list">
				<table>
					<thead>
						<tr>
							<th class="col-date">{locale.date()}</th>
							<th class="col-reporter">{locale.reporter()}</th>
							<th class="col-description">{locale.description()}</th>
						</tr>
					</thead>
					<tbody>
						{#each pageState.alerts as alert (alert.id)}
							{@const reporter = pageState.reporter(alert)}
							<tr>
								<td class="col-date">
									<a href={routes.toolsAlert(alert.id)}>{formatDate(alert.created_at)}</a>
								</td>
								<td class="col-reporter">
									{#if reporter}
										{reporter.name}
									{:else if alert.source === 'telegram' && alert.username}
										@{alert.username}
									{:else}
										{locale.unknownReporter()}
									{/if}
								</td>
								<td class="col-description">
									<span class="description-text">
										{alert.description || locale.noDescription()}
									</span>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{:else}
			<p>{locale.noAlerts()}</p>
		{/if}
	</article>
</AuthWrapper>

<style>
	.alert-list table {
		width: 100%;
		border-collapse: collapse;
	}

	.alert-list th,
	.alert-list td {
		padding: 0.5rem;
		border-bottom: 1px solid light-dark(#ddd, #444);
		text-align: left;
		vertical-align: top;
	}

	.alert-list th {
		font-size: 0.85rem;
		font-weight: 600;
		opacity: 0.7;
		text-transform: uppercase;
	}

	.col-date {
		white-space: nowrap;
	}

	.col-reporter {
		max-width: 12rem;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.col-description {
		width: 100%;
	}

	.description-text {
		display: -webkit-box;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		overflow: hidden;
		overflow-wrap: anywhere;
	}

	.notice {
		font-size: 0.85rem;
		opacity: 0.7;
	}

	.error {
		color: red;
	}

	@media screen and (max-width: 600px) {
		.alert-list th,
		.alert-list td {
			padding: 0.4rem;
		}

		.col-reporter {
			max-width: 8rem;
		}
	}
</style>
