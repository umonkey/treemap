import { startVideoMultipart, completeVideoMultipart } from '$lib/api/panoramas';
import type { IError } from '$lib/types';

const CHUNK_SIZE = 100 * 1024 * 1024; // 100MB
const MAX_CONCURRENCY = 3;
const MAX_RETRIES = 5;
const RETRY_DELAY = 2000;

class VideoUploaderLogic {
	isUploading = $state<boolean>(false);
	uploadProgress = $state<number>(0);
	uploadedBytes = $state<number>(0);
	totalBytes = $state<number>(0);
	error = $state<IError | undefined>(undefined);

	uploadVideo = async (id: string, file: File, onComplete: () => void) => {
		this.isUploading = true;
		this.uploadProgress = 0;
		this.uploadedBytes = 0;
		this.totalBytes = file.size;
		this.error = undefined;

		const partsCount = Math.ceil(file.size / CHUNK_SIZE);
		console.log(
			`Starting multipart upload for ${file.name} (${file.size} bytes), ${partsCount} parts.`
		);

		try {
			const startRes = await startVideoMultipart(id, partsCount);
			if (startRes.status !== 200 || !startRes.data) {
				this.error = startRes.error || {
					code: 'MULTIPART_START_FAILED',
					description: 'Failed to start multipart upload'
				};
				this.isUploading = false;
				return;
			}

			const { upload_id, urls } = startRes.data;
			const completedParts: { part_number: number; etag: string }[] = [];
			const progressPerPart = new Array(partsCount).fill(0);

			const uploadPartWithRetry = async (partNumber: number, url: string) => {
				let attempt = 0;

				while (attempt < MAX_RETRIES) {
					try {
						const etag = await this.performPartUpload(partNumber, url, file, (loaded) => {
							progressPerPart[partNumber - 1] = loaded;
							this.uploadedBytes = progressPerPart.reduce((a, b) => a + b, 0);
							this.uploadProgress = Math.round((this.uploadedBytes / file.size) * 100);
						});

						console.log(`part ${partNumber} uploaded, etag=${etag}`);
						completedParts.push({ part_number: partNumber, etag });
						return;
					} catch (e) {
						attempt++;
						progressPerPart[partNumber - 1] = 0; // Reset progress for this part on failure.
						if (attempt >= MAX_RETRIES) {
							throw e;
						}
						console.warn(`part ${partNumber} failed, retrying (${attempt}/${MAX_RETRIES})...`, e);
						await new Promise((resolve) => setTimeout(resolve, RETRY_DELAY));
					}
				}
			};

			// Upload parts with limited concurrency.
			const queue = urls.map((url, i) => ({ partNumber: i + 1, url }));
			const workers = Array(Math.min(MAX_CONCURRENCY, partsCount))
				.fill(null)
				.map(async () => {
					while (queue.length > 0) {
						const item = queue.shift();
						if (item) {
							await uploadPartWithRetry(item.partNumber, item.url);
						}
					}
				});

			await Promise.all(workers);

			// Sort parts by part_number before sending to complete.
			completedParts.sort((a, b) => a.part_number - b.part_number);
			console.log(`Sending completion request with ${completedParts.length} parts.`);

			const completeRes = await completeVideoMultipart(id, upload_id, completedParts);
			if (completeRes.status === 200 && completeRes.data) {
				onComplete();
			} else {
				this.error = completeRes.error || {
					code: 'MULTIPART_COMPLETE_FAILED',
					description: 'Failed to complete multipart upload'
				};
			}
		} catch (e) {
			console.error('Multipart upload failed:', e);
			this.error = {
				code: 'UPLOAD_FAILED',
				description: e instanceof Error ? e.message : 'Unknown error'
			};
		} finally {
			this.isUploading = false;
		}
	};

	private performPartUpload = (
		partNumber: number,
		url: string,
		file: File,
		onProgress: (loaded: number) => void
	): Promise<string> => {
		const start = (partNumber - 1) * CHUNK_SIZE;
		const end = Math.min(partNumber * CHUNK_SIZE, file.size);
		const chunk = file.slice(start, end);

		return new Promise<string>((resolve, reject) => {
			const xhr = new XMLHttpRequest();
			xhr.open('PUT', url);

			xhr.upload.onprogress = (event) => {
				if (event.lengthComputable) {
					onProgress(event.loaded);
				}
			};

			xhr.onload = () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					const etag = xhr.getResponseHeader('ETag');
					if (etag) {
						resolve(etag);
					} else {
						reject(new Error('ETag missing from response (check CORS ExposeHeaders setting)'));
					}
				} else {
					reject(new Error(`Part ${partNumber} upload failed with status ${xhr.status}`));
				}
			};

			xhr.onerror = () => reject(new Error(`Part ${partNumber} upload failed (network error)`));
			xhr.ontimeout = () => reject(new Error(`Part ${partNumber} upload timed out`));

			xhr.send(chunk);
		});
	};
}

export const componentState = new VideoUploaderLogic();
