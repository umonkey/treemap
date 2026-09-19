<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale as appLocale, getLanguageCode } from '$lib/locale';
	import { formatSize } from '$lib/utils/strings';
	import { ExportPage } from './page.svelte';
	import { locale } from './lang';

	const pageState = new ExportPage();
	const lang = getLanguageCode();

	$effect(() => {
		pageState.reload();
	});
</script>

<svelte:head>
	<title>{locale.title()} — {appLocale.appTitle()}</title>
</svelte:head>

<Dialog title={locale.downloadTitle()}>
	<div>
		{#if lang === 'ru'}
			<p>
				Вы можете скачать всю базу данных (анонимизированную) одним файлом SQLite и делать с ней
				что угодно.
			</p>
			<p>
				Эти данные доступны под <a
					href="https://opendatacommons.org/licenses/odbl/1-0/"
					target="_blank"
					rel="noopener noreferrer">Open Database License (ODbL) v1.0</a
				>. Пожалуйста, при использовании укажите «Trees of Yerevan» и «OpenStreetMap contributors». Любая производная
				база данных должна распространяться под этой же лицензией.
			</p>
		{:else if lang === 'hy'}
			<p>
				Դուք կարող եք ներբեռնել ամբողջ տվյալների բազան (անանունացված) որպես մեկ SQLite ֆայլ և անել
				դրա հետ այն, ինչ ցանկանում եք:
			</p>
			<p>
				Այս տվյալները հասանելի են <a
					href="https://opendatacommons.org/licenses/odbl/1-0/"
					target="_blank"
					rel="noopener noreferrer">Open Database License (ODbL) v1.0</a
				>-ի ներքո: Խնդրում ենք նշել «Trees of Yerevan»-ը և «OpenStreetMap contributors»-ին: Ցանկացած
				ածանցյալ տվյալների բազա պետք է տարածվի նույն լիցենզիայի ներքո:
			</p>
		{:else}
			<p>
				You can download the whole database (anonymized) as a single SQLite file and do whatever you
				want with it.
			</p>
			<p>
				This data is available under the <a
					href="https://opendatacommons.org/licenses/odbl/1-0/"
					target="_blank"
					rel="noopener noreferrer">Open Database License (ODbL) v1.0</a
				>. Please attribute "Trees of Yerevan" and "OpenStreetMap contributors". Any derived
				database must be shared under the same license.
			</p>
		{/if}
	</div>

	{#if pageState.loading}
		<p>{locale.loading()}</p>
	{:else if pageState.error}
		<p>{locale.error(pageState.error.description)}</p>
	{:else if pageState.data.length === 0}
		<p>{locale.noExports()}</p>
	{:else}
		<table>
			<thead>
				<tr>
					<th class="l">{locale.file()}</th>
					<th class="r">{locale.size()}</th>
				</tr>
			</thead>
			<tbody>
				{#each pageState.data as file}
					<tr>
						<td class="l"><a href={file.url} download>{file.name}</a></td>
						<td class="r">{formatSize(file.size)}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</Dialog>

<style>
	table {
		line-height: 1.5;
		width: 100%;
		border-collapse: collapse;
	}

	th,
	td {
		border: 1px solid rgba(128, 128, 128, 0.5);
		padding: 0.5rem 1rem;
	}

	th.l,
	td.l {
		text-align: left;
	}

	th.r,
	td.r {
		text-align: right;
	}

	tr:hover {
		background-color: rgba(128, 128, 128, 0.1);
	}
</style>
