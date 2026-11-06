import { addAlert } from '$lib/api/alerts';
import { uploadSingleFile } from '$lib/api/uploads';
import { mapBus } from '$lib/buses/mapBus';
import { mapState } from '$lib/components/map/MapLibre.svelte.ts';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { solidifyFile } from '$lib/upload';
import { locale } from './lang';

const MAX_PHOTOS = 10;

export type QueuedPhoto = {
	file: File;
	url: string;
};

export class PageState {
	description = $state<string>('');
	photos = $state<QueuedPhoto[]>([]);
	saving = $state<boolean>(false);

	canSend = $derived(this.description.trim().length > 0 && this.photos.length > 0 && !this.saving);

	addFiles = async (files: File[]) => {
		const available = MAX_PHOTOS - this.photos.length;

		if (available <= 0) {
			showError(locale.tooManyPhotos(MAX_PHOTOS));
			return;
		}

		if (files.length > available) {
			showError(locale.tooManyPhotos(MAX_PHOTOS));
		}

		for (const file of files.slice(0, available)) {
			try {
				const stable = await solidifyFile(file);
				this.photos.push({ file: stable, url: URL.createObjectURL(stable) });
			} catch (e) {
				console.error('Failed to queue the alert photo.', e);
				showError(locale.uploadFailed());
			}
		}
	};

	removePhoto = (index: number) => {
		const photo = this.photos[index];

		if (!photo) {
			return;
		}

		URL.revokeObjectURL(photo.url);
		this.photos.splice(index, 1);
	};

	handleSend = async () => {
		if (!this.canSend) {
			return;
		}

		this.saving = true;

		try {
			const files: string[] = [];

			for (const photo of this.photos) {
				const res = await uploadSingleFile(photo.file);

				if (res.status !== 200 || !res.data) {
					console.error(`Error ${res.status} uploading an alert photo.`, res);
					showError(res.error?.description || locale.uploadFailed());
					return;
				}

				files.push(res.data);
			}

			const res = await addAlert({
				lat: mapState.center.lat,
				lon: mapState.center.lng,
				description: this.description.trim(),
				files
			});

			if (res.status >= 200 && res.status < 400 && res.data) {
				mapBus.emit('reload');

				if (typeof navigator !== 'undefined' && navigator.vibrate) {
					navigator.vibrate(50);
				}

				await goto(routes.alertPreview(res.data.id));
			} else {
				console.error(`Error ${res.status} creating an alert.`, res);
				showError(res.error?.description || locale.uploadFailed());
			}
		} finally {
			this.saving = false;
		}
	};

	handleCancel = () => {
		goto(routes.map());
	};

	destroy = () => {
		for (const photo of this.photos) {
			URL.revokeObjectURL(photo.url);
		}

		this.photos = [];
	};
}
