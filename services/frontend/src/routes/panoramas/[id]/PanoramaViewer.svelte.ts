import type { MapillaryImage } from '$lib/api/mapillary';
import 'pannellum';

class PanoramaViewerLogic {
	viewer: Pannellum.Viewer | null = null;
	yaw = $state(0);

	init = (container: HTMLElement, image: MapillaryImage) => {
		if (this.viewer) {
			this.viewer.destroy();
			this.viewer = null;
		}

		this.yaw = 0;

		if (!image.url) return;

		this.viewer = window.pannellum.viewer(container, {
			type: 'equirectangular',
			panorama: image.url,
			autoLoad: true,
			compass: true,
			northOffset: image.compass_angle,
			showControls: false,
			mouseZoom: true,
			keyboardZoom: true
		});

		this.viewer.on('load', () => {
			this.viewer?.resize();
		});

		this.viewer.on('animatefinished', (data) => {
			this.yaw = data.yaw;
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
