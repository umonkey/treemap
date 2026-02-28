<script lang="ts">
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { loadUploads } from './UploadsList';
	import { formatDate } from '$lib/utils/strings';
	import type { IUpload } from '$lib/db';

	let uploads = $state<IUpload[]>([]);

	$effect(() => {
		return loadUploads((data) => {
			uploads = data;
		});
	});

	const formatSize = (bytes: number) => {
		return (bytes / 1024 / 1024).toFixed(2) + ' MB';
	};
</script>

<table class="uploads-table">
	<thead>
		<tr>
			<th>{locale.uploadsDate()}</th>
			<th>{locale.treeShortTitle()}</th>
			<th>{locale.uploadsStatus()}</th>
			<th>{locale.uploadsSize()}</th>
		</tr>
	</thead>
	<tbody>
		{#each uploads as upload}
			<tr>
				<td>{formatDate(upload.created_at / 1000)}</td>
				<td>
					<a href={routes.treeDetails(upload.tree_id)}>
						{upload.tree_id}
					</a>
				</td>
				<td class="status-{upload.status}">{upload.status}</td>
				<td>{formatSize(upload.image.size)}</td>
			</tr>
		{/each}
	</tbody>
</table>

<style>
	.uploads-table {
		width: 100%;
		border-collapse: collapse;
		margin-top: var(--gap);
	}

	.uploads-table th,
	.uploads-table td {
		text-align: left;
		padding: 10px;
	}

	.uploads-table th {
		font-weight: bold;
		color: var(--text-color);
	}

	.uploads-table tr:last-child td {
		border-bottom: none;
	}

	.status-pending {
		color: orange;
	}

	.status-uploading {
		color: blue;
	}

	.status-completed {
		color: green;
	}

	.status-failed {
		color: red;
	}
</style>
