import { db } from './db';
import { apiClient } from './api';

// Delay between file upload attempts.
const DELAY = 1000;

let isProcessing = false;

/**
 * Add a photo to the upload queue.
 */
export async function addPhotoToUploadQueue(tree_id: string | number, file: File) {
	await db.uploads.add({
		tree_id: tree_id.toString(),
		status: 'pending',
		image: file,
		created_at: Date.now(),
		retry_count: 0,
		file_id: null
	});

	// Trigger processing.
	processUploadQueue();
}

/**
 * Process the upload queue.
 */
export async function processUploadQueue() {
	if (isProcessing) {
		return;
	}

	if (typeof navigator !== 'undefined' && !navigator.onLine) {
		return;
	}

	isProcessing = true;

	try {
		while (true) {
			const pending = await db.transaction('rw', db.uploads, async () => {
				const item = await db.uploads
					.where('status')
					.anyOf('pending', 'failed')
					.filter((u) => u.retry_count < 5)
					.first();

				if (item && item.id) {
					await db.uploads.update(item.id, { status: 'uploading' });
				}

				return item;
			});

			if (!pending || !pending.id) {
				break;
			}

			try {
				const file_id = await uploadSingleFile(pending.tree_id, pending.image);
				await db.uploads.update(pending.id, {
					status: 'completed',
					file_id
				});
			} catch (e) {
				console.error(`[upload] Failed to upload ${pending.id}:`, e);
				await db.uploads.update(pending.id, {
					status: 'failed',
					retry_count: pending.retry_count + 1
				});

				// Wait a bit before retrying or moving to next.
				await new Promise((resolve) => setTimeout(resolve, DELAY));
			}
		}
	} finally {
		isProcessing = false;
	}
}

/**
 * Upload a single file and attach it to a tree.
 */
export async function uploadSingleFile(tree_id: string, blob: Blob): Promise<string> {
	const res = await apiClient.uploadSingleFile(blob);
	if (res.status !== 200 || !res.data) {
		throw new Error(res.error?.description || 'Failed to upload file');
	}

	const file_id = res.data;

	// 2. Attach to tree.
	const attachRes = await apiClient.addPhotos(tree_id, [file_id]);
	if (attachRes.status !== 200) {
		throw new Error(attachRes.error?.description || 'Failed to attach photo to tree');
	}

	return file_id;
}

// Start processing on load if online.
if (typeof window !== 'undefined') {
	window.addEventListener('online', () => {
		processUploadQueue();
	});

	// Initial check.
	processUploadQueue();
}
