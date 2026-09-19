import { getMe, updateSettings } from '$lib/api/users';
import { showError, showWarning } from '$lib/errors';
import type { IMeResponse } from '$lib/types';
import { locale } from './DisplayNameInput.lang';

export class SettingsPage {
	loading = $state<boolean>(true);
	saving = $state<boolean>(false);
	error = $state<string | null>(null);
	data = $state<IMeResponse | null>(null);
	files = $state<string[]>([]);
	name = $state<string>('');

	reload = async () => {
		this.loading = true;
		this.error = null;

		try {
			const { status, data: d, error: e } = await getMe();

			if (status === 200 && d) {
				this.data = d;
				this.name = d.user.name;
			} else if (e) {
				this.error = e.description;
			}
		} finally {
			this.loading = false;
		}
	};

	handleNameBlur = async (name: string) => {
		if (!name.trim()) {
			showWarning(locale.displayNameRequired());
			this.name = this.data?.user.name ?? '';
			return;
		}
		if (name === this.data?.user.name) return;
		await this.save({ name });
	};

	handleFileBusy = (value: boolean) => {
		this.saving = value;
	};

	handleFileChange = (value: string[]) => {
		const previous = this.files[0];
		this.files = value;
		const next = value[0];
		if (next && next !== previous) void this.save({ picture: next });
	};

	private save = async (payload: { name?: string; picture?: string }) => {
		this.saving = true;
		try {
			const { status, error } = await updateSettings(payload);
			if (status === 202) {
				if (payload.name !== undefined && this.data) {
					this.data = { ...this.data, user: { ...this.data.user, name: payload.name } };
				}
			} else if (error) {
				showError(error.description);
			}
		} finally {
			this.saving = false;
		}
	};
}
