import type { MapillaryImage } from '$lib/api/mapillary';
import 'pannellum';

class PanoramaViewerLogic {
	viewer: Pannellum.Viewer | null = null;
	yaw = $state(0);
	onMove?: (angle: number) => void;

	init = (
		container: HTMLElement,
		image: MapillaryImage,
		initialYaw: number = 0,
		onMove?: (angle: number) => void
	) => {
		if (this.viewer) {
			this.viewer.destroy();
			this.viewer = null;
		}

		this.yaw = initialYaw;
		this.onMove = onMove;

		if (!image.url) return;

		this.viewer = window.pannellum.viewer(container, {
			type: 'equirectangular',
			panorama: image.url,
			autoLoad: true,
			compass: true,
			northOffset: image.compass_angle,
			yaw: initialYaw,
			showControls: false,
			mouseZoom: true,
			keyboardZoom: true
		});

		this.viewer.on('load', () => {
			this.viewer?.resize();
			this.onMove?.(this.yaw);
		});

		this.viewer.on('animatefinished', (data) => {
			this.yaw = data.yaw;
			this.onMove?.(this.yaw);
		});
	};

	destroy = () => {
		if (this.viewer) {
			this.viewer.destroy();
			this.viewer = null;
		}
	};
}

export const componentState = new PanoramaViewerLogic();
