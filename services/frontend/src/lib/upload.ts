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

import { addPhotos, createUploadTicket, finishUpload, putFileToStorage } from '$lib/api/uploads';
import { uploadBus } from '$lib/buses/upload';
import { showError } from '$lib/errors';
import { uploadStore } from '$lib/stores/upload';
import { setUploading } from '$lib/stores/upload';
import { isDataSaving, isLowPower } from '$lib/utils/device';
import { createThumbnail } from '$lib/utils/images';
import { get } from 'svelte/store';
import { db, type IUpload, type UploadStage } from './db';

// Maximum number of attempts per queue run.
const MAX_RETRY_ATTEMPTS = 5;
// Base delay between attempts; doubles on every attempt and is capped.
const RETRY_BASE_DELAY = 5 * 1000;
const RETRY_MAX_DELAY = 60 * 1000;
const RETRY_JITTER = 0.2;
const SUCCESS_DELAY = 2 * 1000;

/**
 * Get the delay before the next upload attempt.
 *
 * Five seconds, doubled per attempt and capped at one minute, with plus or
 * minus twenty percent jitter so devices that failed together do not retry
 * together.
 */
export function getRetryDelay(attempt: number): number {
	const exponent = Math.max(1, Math.floor(attempt)) - 1;
	const delay = Math.min(RETRY_BASE_DELAY * 2 ** exponent, RETRY_MAX_DELAY);
	const jitter = delay * RETRY_JITTER;

	return Math.round(delay + (Math.random() * 2 - 1) * jitter);
}

/**
 * Whether a failed API call is worth retrying.
 *
 * Server errors, rate limits, request timeouts, and transport failures are
 * transient. Other client errors will not fix themselves.
 */
export function isRetryableError(status: number, code?: string): boolean {
	if (code === 'network_error' || code === 'upload_error') {
		return true;
	}

	if (status >= 500) {
		return true;
	}

	return status === 408 || status === 429;
}

/**
 * An upload failure that remembers which stage failed and how to classify it.
 */
export class UploadError extends Error {
	status: number;
	code?: string;
	stage: UploadStage;

	constructor(message: string, status: number, stage: UploadStage, code?: string) {
		super(message);
		this.name = 'UploadError';
		this.status = status;
		this.stage = stage;
		this.code = code;
	}
}

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
export async function addPhotoToUploadQueue(tree_id: string, file: File) {
	if (!tree_id) {
		showError('Could not add the photo, please try again.');
		return;
	}

	// 1. Save the original image as soon as possible to avoid InvalidBlob errors.
	const id = await db.uploads.add({
		tree_id: tree_id.toString(),
		status: 'pending',
		image: file,
		thumbnail: undefined,
		created_at: Date.now(),
		retry_count: 0,
		file_id: null,
		progress: 0,
		stage: 'new'
	});

	autoStartUpload();

	// 2. Generate thumbnail in the background.
	// We don't await this because we want to return to the UI immediately.
	createThumbnail(file, 200)
		.then((thumbnail) => {
			db.uploads.update(id, { thumbnail });
		})
		.catch((e) => {
			console.error('Failed to create thumbnail for upload', id, e);
		});
}

/** Save a stable in-memory copy of a File (mobile File-invalidation workaround). */
export async function solidifyFile(file: File, name = file.name): Promise<File> {
	const buffer = await file.arrayBuffer();
	return new File([buffer], name, { type: file.type });
}

/** Solidify a photo and add it to the upload queue, reporting failures to the user. */
export async function queuePhoto(treeId: string, file: File, name = file.name): Promise<void> {
	try {
		await addPhotoToUploadQueue(treeId, await solidifyFile(file, name));
	} catch (e) {
		console.error('Failed to append file:', e);
		showError(
			`Failed to add photo "${file.name}": ${e instanceof Error ? e.message : 'Unknown error'}`
		);
	}
}

// Trigger processing if auto-upload is enabled.
export const autoStartUpload = async () => {
	await db.uploads.where('status').equals('completed').delete();

	// Remove invalid entries.
	await db.uploads.filter((u) => !u.tree_id).delete();

	if (get(uploadStore).autoupload) {
		if (isDataSaving()) {
			console.debug('[upload] Data saving mode is enabled, auto-upload skipped.');
			return;
		}

		if (await isLowPower()) {
			console.debug('[upload] Low power mode is detected, auto-upload skipped.');
			return;
		}

		console.debug('Auto-upload enabled, triggering.');
		processUploadQueue();
	} else {
		console.debug('Auto-upload disabled.');
	}
};

/**
 * Restart the upload queue by resetting failed uploads.
 *
 * Keeps the stage and file id so a manual retry resumes instead of
 * re-transferring an already uploaded file.
 */
export async function restartUploadQueue() {
	await db.uploads
		.where('status')
		.anyOf('failed', 'uploading')
		.modify((u) => {
			u.status = 'pending';
			u.retry_count = 0;
			u.permanently_failed = false;
		});
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

	// Auto-restart previously failed uploads, except the ones that need a
	// manual retry because their error is not transient.
	await db.uploads
		.where('status')
		.anyOf('failed', 'uploading')
		.and((u) => !u.permanently_failed)
		.modify({ status: 'pending', retry_count: 0 });

	try {
		while (true) {
			const pending = await db.transaction('rw', db.uploads, async () => {
				const item = await db.uploads
					.orderBy('created_at')
					.reverse()
					.filter(
						(u) =>
							(u.status === 'pending' || u.status === 'failed') &&
							!u.permanently_failed &&
							u.retry_count < MAX_RETRY_ATTEMPTS
					)
					.first();

				if (item?.id) {
					await db.uploads.update(item.id, {
						status: 'uploading',
						progress: 0
					});
				}

				return item;
			});

			if (!pending?.id) {
				break;
			}

			const uploadId = pending.id;

			try {
				await processUpload(pending, (progress) => {
					db.uploads.update(uploadId, { progress });
				});

				await db.uploads.delete(uploadId);

				uploadBus.emit('success', pending.tree_id);

				// Let slow networks cool down.
				const delay = getSuccessDelay();

				if (delay > 0) {
					await new Promise((resolve) => setTimeout(resolve, delay));
				}
			} catch (e) {
				const uploadError = e instanceof UploadError ? e : undefined;
				const status = uploadError?.status ?? 500;
				const code = uploadError?.code;
				const stage = uploadError?.stage ?? pending.stage ?? 'new';
				const attempt = pending.retry_count + 1;
				const retryable = isRetryableError(status, code);
				const terminal = !retryable || attempt >= MAX_RETRY_ATTEMPTS;

				await db.uploads.update(uploadId, {
					status: 'failed',
					retry_count: attempt,
					...(retryable ? {} : { permanently_failed: true })
				});

				if (terminal) {
					console.error(
						`[upload] Upload ${uploadId} failed at stage "${stage}" after ${attempt} attempt(s):`,
						e
					);
				} else {
					console.warn(
						`[upload] Upload ${uploadId} failed at stage "${stage}" (attempt ${attempt}), retrying:`,
						e
					);
				}

				// No point in waiting when this record will not be picked again.
				if (terminal) {
					continue;
				}

				await new Promise((resolve) => setTimeout(resolve, getRetryDelay(attempt)));
			}
		}
	} finally {
		isProcessing = false;
		setUploading(false);
	}
}

/**
 * Upload a single queue item and attach it to a tree.
 *
 * Resumes at the last persisted stage: the bytes are never transferred again
 * once the file id is stored, and a finished file is only attached again.
 */
export async function processUpload(
	item: IUpload,
	onProgress?: (p: number) => void
): Promise<string> {
	let stage: UploadStage = item.stage ?? 'new';
	let file_id = item.file_id;
	let ticketResetted = false;

	while (true) {
		if (stage === 'new') {
			const ticketRes = await createUploadTicket(
				item.image.size,
				item.image.type || 'application/octet-stream'
			);

			if (ticketRes.status !== 200 || !ticketRes.data) {
				throw new UploadError(
					ticketRes.error?.description || 'Failed to create an upload ticket',
					ticketRes.status,
					'new',
					ticketRes.error?.code
				);
			}

			const { id, url } = ticketRes.data;
			const putRes = await putFileToStorage(url, item.image, onProgress);

			if (putRes.error) {
				throw new UploadError(
					putRes.error.description || 'Failed to upload the file',
					putRes.status,
					'new',
					putRes.error.code
				);
			}

			file_id = id;

			// Persist the ticket together with the stage, never before: a failed
			// PUT must leave the record clean so the next attempt requests a new
			// ticket instead of reusing an expired URL.
			if (item.id !== undefined) {
				await db.uploads.update(item.id, { file_id, stage: 'uploaded' });
			}

			stage = 'uploaded';
			continue;
		}

		if (stage === 'uploaded') {
			if (!file_id) {
				// Inconsistent record: there is no ticket to finish.
				stage = 'new';
				continue;
			}

			const finishRes = await finishUpload(file_id);

			if (finishRes.status === 404) {
				// The ticket is gone (for example, the file row was removed).
				// Drop it and re-upload, at most once per attempt.
				if (ticketResetted) {
					throw new UploadError(
						'The upload ticket is gone',
						finishRes.status,
						'uploaded',
						finishRes.error?.code
					);
				}

				ticketResetted = true;
				file_id = null;
				stage = 'new';

				if (item.id !== undefined) {
					await db.uploads.update(item.id, { file_id: null, stage: 'new' });
				}

				continue;
			}

			if (finishRes.status >= 400) {
				throw new UploadError(
					finishRes.error?.description || 'Failed to finalize the upload',
					finishRes.status,
					'uploaded',
					finishRes.error?.code
				);
			}

			if (item.id !== undefined) {
				await db.uploads.update(item.id, { stage: 'finished' });
			}

			stage = 'finished';
			continue;
		}

		// stage === 'finished'
		if (!file_id) {
			// Inconsistent record: nothing to attach.
			stage = 'new';
			continue;
		}

		// Attachment is idempotent server-side (INSERT OR IGNORE plus a UNIQUE
		// constraint), so retrying after a successful attach is harmless.
		const attachRes = await addPhotos(item.tree_id, [file_id]);

		if (attachRes.status >= 400) {
			throw new UploadError(
				attachRes.error?.description || 'Failed to attach the photo to the tree',
				attachRes.status,
				'finished',
				attachRes.error?.code
			);
		}

		return file_id;
	}
}

// Start processing on load if online.
if (typeof window !== 'undefined') {
	window.addEventListener('online', () => {
		autoStartUpload();
	});

	// Initial check.
	autoStartUpload();
}
