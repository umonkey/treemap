import { uploadSingleFile } from '$lib/api/uploads';
import { showError } from '$lib/errors';

export type FileUploaderItem = {
	file: File;
	uploading: boolean;
	error: boolean;
};

export class FileUploader {
	items = $state<FileUploaderItem[]>([]);
	uploads = $state<string[]>([]);
	busy = $state<boolean>(false);

	handleChange = (event: Event) => {
		const target = event.target as HTMLInputElement;
		const files = target.files;
		if (files && files.length > 0) {
			const fileList = Array.from(files);
			target.value = '';
			for (const file of fileList) {
				const idx = this.items.length;
				this.items = [...this.items, { file, uploading: false, error: false }];
				this.handleRetry(idx);
			}
		}
		console.debug(`[FileUploader] Have ${this.items.length} files selected.`);
	};

	handleRetry = (idx: number) => {
		this.busy = true;
		const file = this.items[idx].file;

		// Mark the file as uploading.
		const uploading = [...this.items];
		uploading[idx] = { ...uploading[idx], uploading: true, error: false };
		this.items = uploading;

		uploadSingleFile(file)
			.then((res) => {
				if (res.status === 200 && res.data) {
					const uploadId = res.data;
					this.uploads = [...this.uploads, uploadId];
					console.info(`[FileUploader] File uploaded successfully: ${file.name}, id=${uploadId}`);
					const done = [...this.items];
					done[idx] = { ...done[idx], uploading: false, error: false };
					this.items = done;
				} else {
					console.error(`[FileUploader] Failed to upload file: ${file.name}`, res);
					showError(
						`Failed to upload file "${file.name}": ${res.error?.description || 'Unknown error'}`
					);
					const failed = [...this.items];
					failed[idx] = { ...failed[idx], uploading: false, error: true };
					this.items = failed;
				}
			})
			.catch((e) => {
				console.error(`[FileUploader] Error uploading file: ${file.name}`, e);
				showError(
					`Error uploading file "${file.name}": ${e instanceof Error ? e.message : 'Unknown error'}`
				);
				const failed = [...this.items];
				failed[idx] = { ...failed[idx], uploading: false, error: false };
				this.items = failed;
			})
			.finally(() => {
				this.busy = false;
			});
	};
}
