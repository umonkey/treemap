import type { IResponse, IUploadTicket } from '$lib/types';
import { getAuthHeaders, request } from './client';

/**
 * This is deprecated.
 * Please use uploadSingleFile for now.
 **/
export async function uploadFile(tree: string, file: File): Promise<IResponse<void>> {
	const headers: HeadersInit = {
		'Content-Type': 'application/json',
		...getAuthHeaders()
	};

	const buffer = await file.arrayBuffer();
	const body = new Blob([buffer], { type: file.type });

	return await request('POST', `v1/trees/${tree}/files`, {
		body,
		headers
	});
}

// Add previously uploaded files as photos to a tree.
export async function addPhotos(tree_id: string, files: string[]): Promise<IResponse<void>> {
	const headers: HeadersInit = {
		'Content-Type': 'application/json',
		...getAuthHeaders()
	};

	return await request('POST', `v1/trees/${tree_id}/photos`, {
		body: JSON.stringify({ files }),
		headers
	});
}

/**
 * This is how you upload all files.
 * The response is the file id, which is then used for adding photos to trees.
 */
export async function uploadSingleFile(
	file: Blob,
	onProgress?: (p: number) => void
): Promise<IResponse<string>> {
	const ticketRes = await request<IUploadTicket>('POST', 'v1/upload', {
		body: JSON.stringify({ size: file.size }),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});

	if (ticketRes.status !== 200 || !ticketRes.data) {
		return {
			status: ticketRes.status,
			data: undefined,
			error: ticketRes.error
		};
	}

	const { id, url } = ticketRes.data;

	try {
		const start = performance.now();

		const uploadResult = await new Promise<{
			ok: boolean;
			status: number;
			statusText: string;
		}>((resolve, reject) => {
			const xhr = new XMLHttpRequest();
			xhr.open('PUT', url);

			if (onProgress) {
				xhr.upload.onprogress = (e) => {
					if (e.lengthComputable) {
						const percentComplete = Math.round((e.loaded / e.total) * 100);
						onProgress(percentComplete);
					}
				};
			}

			xhr.onload = () => {
				resolve({
					ok: xhr.status >= 200 && xhr.status < 300,
					status: xhr.status,
					statusText: xhr.statusText
				});
			};

			xhr.onerror = () => {
				reject(new Error('Network error during upload'));
			};

			xhr.send(file);
		});

		const duration = Math.round(performance.now() - start);
		console.log(`[api] File upload took ${duration} ms, status=${uploadResult.status}`);

		if (!uploadResult.ok) {
			return {
				status: uploadResult.status,
				data: undefined,
				error: {
					code: 'upload_error',
					description: `Failed to upload file to S3: ${uploadResult.statusText}`
				}
			};
		}
	} catch (e) {
		return {
			status: 500,
			data: undefined,
			error: {
				code: 'upload_error',
				description: (e as Error).message
			}
		};
	}

	const finishRes = await request<void>('POST', `v1/upload/${id}/finish`, {
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});

	if (finishRes.status >= 400) {
		return {
			status: finishRes.status,
			data: undefined,
			error: finishRes.error
		};
	}

	return {
		status: 200,
		data: id,
		error: undefined
	};
}

export async function deleteFile(id: string): Promise<IResponse<void>> {
	return await request('DELETE', `v1/files/${id}`, {
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}
