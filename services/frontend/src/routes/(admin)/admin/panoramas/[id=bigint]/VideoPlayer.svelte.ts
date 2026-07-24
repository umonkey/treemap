import { getPanoramaWebVideo } from '$lib/api/panoramas';
import type { IError } from '$lib/types';

class VideoPlayerLogic {
	videoUrl = $state<string | null>(null);
	isLoading = $state<boolean>(false);
	error = $state<IError | null>(null);

	load = async (id: string) => {
		this.isLoading = true;
		this.error = null;
		this.videoUrl = null;

		const res = await getPanoramaWebVideo(id);
		this.isLoading = false;

		if (res.status === 200 && res.data) {
			this.videoUrl = res.data.url;
		} else {
			this.error = res.error || {
				code: 'WEB_VIDEO_FAILED',
				description: 'Failed to load web video'
			};
		}
	};
}

export const componentState = new VideoPlayerLogic();
