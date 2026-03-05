/**
 * This file contains the background image uploader.
 *
 * The goal is to unblock the UI asap so that the user doesn't have to wait
 * for the files to finish uploading to continue operating.
 *
 * To start an upload, just call addPhotoToUploadQueue() with the tree and file info.
 *
 * Additionally, when the app starts, call processUploadQueue() to restart any
 * previously failed attempts.
 */

import { db } from './db';
import { apiClient } from './api';
import { uploadBus } from '$lib/buses/upload';
import { uploadStore } from '$lib/stores/upload';
import { get } from 'svelte/store';
import {
	incrementUploadCount,
	decrementUploadCount,
	resetUploadCount,
	setUploading
} from '$lib/stores/upload';

// Delay between file upload attempts.
const ERROR_DELAY = 5 * 1000;
const SUCCESS_DELAY = 2 * 1000;

/**
 * Get the delay to wait after a successful upload.
 */
function getSuccessDelay(): number {
	if (typeof navigator !== 'undefined' && 'connection' in navigator) {
		const conn = (navigator as Navigator & { connection?: { type?: string } }).connection;

		if (conn?.type === 'wifi' || conn?.type === 'ethernet') {
			return 0;
		}

		if (conn?.type === 'cellular') {
			return SUCCESS_DELAY;
		}
	}

	return SUCCESS_DELAY;
}

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
		file_id: null,
		progress: 0
	});

	incrementUploadCount();
	autoStartUpload();
}

// Trigger processing if auto-upload is enabled.
export const autoStartUpload = async () => {
	await db.uploads.where('status').equals('completed').delete();

	if (get(uploadStore).autoupload) {
		console.debug('Auto-upload enabled, triggering.');
		processUploadQueue();
	} else {
		console.debug('Auto-upload disabled.');
	}
};

/**
 * Restart the upload queue by resetting failed uploads.
 */
export async function restartUploadQueue() {
	await db.uploads
		.where('status')
		.anyOf('failed', 'uploading')
		.modify({ status: 'pending', retry_count: 0 });
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

	console.debug('Going to process the upload queue.');

	isProcessing = true;
	setUploading(true);

	// Auto-restart previously failed uploads.
	await db.uploads
		.where('status')
		.anyOf('failed', 'uploading')
		.modify({ status: 'pending', retry_count: 0 });

	try {
		while (true) {
			const pending = await db.transaction('rw', db.uploads, async () => {
				const item = await db.uploads
					.where('status')
					.anyOf('pending', 'failed')
					.filter((u) => u.retry_count < 5)
					.first();

				if (item && item.id) {
					await db.uploads.update(item.id, { status: 'uploading', progress: 0 });
				}

				return item;
			});

			if (!pending || !pending.id) {
				resetUploadCount();
				break;
			}

			try {
				await uploadSingleFile(pending.tree_id, pending.image, pending.id);

				await db.uploads.delete(pending.id);

				decrementUploadCount();

				uploadBus.emit('success', pending.tree_id);

				// Let slow networks cool down.
				const delay = getSuccessDelay();

				if (delay > 0) {
					await new Promise((resolve) => setTimeout(resolve, delay));
				}
			} catch (e) {
				console.error(`[upload] Failed to upload ${pending.id}:`, e);

				await db.uploads.update(pending.id, {
					status: 'failed',
					retry_count: pending.retry_count + 1
				});

				// Wait a bit before retrying or moving to next.
				await new Promise((resolve) => setTimeout(resolve, ERROR_DELAY));
			}
		}
	} finally {
		isProcessing = false;
		setUploading(false);
	}
}

/**
 * Upload a single file and attach it to a tree.
 */
export async function uploadSingleFile(
	tree_id: string,
	blob: Blob,
	upload_id?: number
): Promise<string> {
	const res = await apiClient.uploadSingleFile(blob, (progress) => {
		if (upload_id) {
			db.uploads.update(upload_id, { progress });
		}
	});
	if (res.status !== 200 || !res.data) {
		throw new Error(res.error?.description || 'Failed to upload file');
	}

	const file_id = res.data;

	// 2. Attach to tree.
	const attachRes = await apiClient.addPhotos(tree_id, [file_id]);
	if (attachRes.status >= 400) {
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
	autoStartUpload();
}
