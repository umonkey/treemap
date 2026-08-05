import { startVideoMultipart, completeVideoMultipart } from '$lib/api/panoramas';
import type { IError } from '$lib/types';

const CHUNK_SIZE = 100 * 1024 * 1024; // 100MB
const MAX_CONCURRENCY = 3;
const MAX_RETRIES = 5;
const RETRY_DELAY = 2000;

export class VideoUploaderState {
	isUploading = $state<boolean>(false);
	uploadProgress = $state<number>(0);
	uploadedBytes = $state<number>(0);
	totalBytes = $state<number>(0);
	error = $state<IError | undefined>(undefined);
	private uploadId = $state<string | undefined>(undefined);
	private urls = $state<string[]>([]);
	private completedParts = $state<{ part_number: number; etag: string }[]>([]);
	private activeFile = $state<File | undefined>(undefined);
	private activePanoramaId = $state<string | undefined>(undefined);

	reset = () => {
		this.isUploading = false;
		this.uploadProgress = 0;
		this.uploadedBytes = 0;
		this.totalBytes = 0;
		this.error = undefined;
		this.uploadId = undefined;
		this.urls = [];
		this.completedParts = [];
		this.activeFile = undefined;
		this.activePanoramaId = undefined;
	};

	uploadVideo = async (id: string, file: File, onComplete: () => void) => {
		const isResuming = this.uploadId && this.activeFile === file && this.activePanoramaId === id;

		this.isUploading = true;
		this.error = undefined;
		this.activeFile = file;
		this.activePanoramaId = id;
		this.totalBytes = file.size;

		const partsCount = Math.ceil(file.size / CHUNK_SIZE);

		try {
			if (!isResuming) {
				console.log(
					`Starting multipart upload for ${file.name} (${file.size} bytes), ${partsCount} parts.`
				);
				const startRes = await startVideoMultipart(id, partsCount);
				if (startRes.status !== 200 || !startRes.data) {
					this.error = startRes.error || {
						code: 'MULTIPART_START_FAILED',
						description: 'Failed to start multipart upload'
					};
					this.isUploading = false;
					return;
				}
				this.uploadId = startRes.data.upload_id;
				this.urls = startRes.data.urls;
				this.completedParts = [];
			} else {
				console.log(`Resuming multipart upload for ${file.name}, ${partsCount} parts.`);
			}

			const progressPerPart = new Array(partsCount).fill(0);
			// Initialize progress for already completed parts.
			for (const part of this.completedParts) {
				const partIndex = part.part_number - 1;
				const partStart = partIndex * CHUNK_SIZE;
				const partEnd = Math.min((partIndex + 1) * CHUNK_SIZE, file.size);
				progressPerPart[partIndex] = partEnd - partStart;
			}
			this.uploadedBytes = progressPerPart.reduce((a, b) => a + b, 0);
			this.uploadProgress = Math.round((this.uploadedBytes / file.size) * 100);

			const uploadPartWithRetry = async (partNumber: number, url: string) => {
				// Skip already completed parts.
				if (this.completedParts.find((p) => p.part_number === partNumber)) {
					return;
				}

				let attempt = 0;

				while (attempt < MAX_RETRIES) {
					try {
						const etag = await this.performPartUpload(partNumber, url, file, (loaded) => {
							progressPerPart[partNumber - 1] = loaded;
							this.uploadedBytes = progressPerPart.reduce((a, b) => a + b, 0);
							this.uploadProgress = Math.round((this.uploadedBytes / file.size) * 100);
						});

						console.log(`part ${partNumber} uploaded, etag=${etag}`);
						this.completedParts.push({ part_number: partNumber, etag });
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
			const queue = this.urls.map((url, i) => ({ partNumber: i + 1, url }));
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
			const finalParts = [...this.completedParts].sort((a, b) => a.part_number - b.part_number);
			console.log(`Sending completion request with ${finalParts.length} parts.`);

			const completeRes = await completeVideoMultipart(id, this.uploadId!, finalParts);
			if (completeRes.status === 200 && completeRes.data) {
				this.reset();
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
