import {
	getPanoramaTrackUploadUrl,
	uploadPanoramaTrackFile,
	finishPanoramaTrackUpload
} from '$lib/api/panoramas';
import type { IError } from '$lib/types';

class TrackUploaderLogic {
	isUploading = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);

	reset = () => {
		this.isUploading = false;
		this.error = undefined;
	};

	uploadTrack = async (id: string, file: File, onComplete: () => void) => {
		this.isUploading = true;
		this.error = undefined;

		try {
			const urlRes = await getPanoramaTrackUploadUrl(id);
			if (urlRes.status !== 200 || !urlRes.data) {
				this.error = urlRes.error || {
					code: 'TRACK_UPLOAD_URL_FAILED',
					description: 'Failed to get track upload URL'
				};
				this.isUploading = false;
				return;
			}

			const uploadRes = await uploadPanoramaTrackFile(urlRes.data.url, file);
			if (!uploadRes.ok) {
				throw new Error(`Failed to upload track file: ${uploadRes.statusText}`);
			}

			const finishRes = await finishPanoramaTrackUpload(id);
			if (finishRes.status === 200 && finishRes.data) {
				this.reset();
				onComplete();
			} else {
				this.error = finishRes.error || {
					code: 'TRACK_FINISH_FAILED',
					description: 'Failed to verify track upload'
				};
			}
		} catch (e) {
			console.error('Track upload failed:', e);
			this.error = {
				code: 'UPLOAD_FAILED',
				description: e instanceof Error ? e.message : 'Unknown error'
			};
		} finally {
			this.isUploading = false;
		}
	};
}

export const componentState = new TrackUploaderLogic();
