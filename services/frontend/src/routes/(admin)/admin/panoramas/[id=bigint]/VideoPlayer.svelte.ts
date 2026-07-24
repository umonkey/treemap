import { Viewer } from '@photo-sphere-viewer/core';
import { VideoPlugin, events } from '@photo-sphere-viewer/video-plugin';
import { EquirectangularVideoAdapter } from '@photo-sphere-viewer/equirectangular-video-adapter';
import { getPanoramaWebVideo } from '$lib/api/panoramas';
import type { IError } from '$lib/types';
import '@photo-sphere-viewer/core/index.css';
import '@photo-sphere-viewer/video-plugin/index.css';

export class VideoPlayerState {
	videoUrl = $state<string | null>(null);
	isLoading = $state<boolean>(false);
	error = $state<IError | null>(null);
	currentTime = $state<number>(0);

	private viewer: Viewer | null = null;

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

	init = (container: HTMLElement, url: string) => {
		if (this.viewer) {
			this.viewer.destroy();
			this.viewer = null;
		}
		if (!container || !url) return;

		this.viewer = new Viewer({
			container,
			adapter: EquirectangularVideoAdapter,
			panorama: { source: url },
			plugins: [VideoPlugin],
			navbar: false,
			keyboard: false,
			mousewheel: false
		});

		(this.viewer as unknown as { resize: () => void }).resize();

		const videoPlugin = this.viewer.getPlugin<VideoPlugin>('video');
		videoPlugin?.addEventListener(events.ProgressEvent.type, (e) => {
			this.currentTime = e.time;
		});
		this.viewer.addEventListener('click', () => {
			videoPlugin?.playPause();
		});
	};

	destroy = () => {
		if (this.viewer) {
			this.viewer.destroy();
			this.viewer = null;
		}
	};

	play = () => {
		const videoPlugin = this.viewer?.getPlugin<VideoPlugin>('video');
		videoPlugin?.play();
	};

	pause = () => {
		const videoPlugin = this.viewer?.getPlugin<VideoPlugin>('video');
		videoPlugin?.pause();
	};

	rotate = (yaw: number, pitch: number) => {
		this.viewer?.rotate({ yaw, pitch });
	};

	getCurrentTime = (): number => {
		const videoPlugin = this.viewer?.getPlugin<VideoPlugin>('video');
		return videoPlugin?.getTime() ?? 0;
	};
}
