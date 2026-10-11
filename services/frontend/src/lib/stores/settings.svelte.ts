import { getSettings } from '$lib/api/settings';
import { DEFAULT_BBOX } from '$lib/constants';
import { bboxCenter } from '$lib/utils/geo';

class SettingsState {
	name = $state('');
	description = $state<string | null>(null);
	bbox = $state<[number, number, number, number]>(DEFAULT_BBOX);
	center = $derived(bboxCenter(this.bbox));
	loaded = $state(false);

	load = async () => {
		const res = await getSettings();

		if (res.data) {
			this.name = res.data.name;
			this.description = res.data.description;
			this.bbox = res.data.bbox ?? DEFAULT_BBOX;
		} else {
			console.warn('[settings] Failed to load instance settings.', res.error);
		}

		this.loaded = true;
	};
}

export const settingsState = new SettingsState();
