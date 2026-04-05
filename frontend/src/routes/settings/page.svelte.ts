import { getMe, updateSettings } from '$lib/api/users';
import { goto, routes } from '$lib/routes';
import type { IMeResponse } from '$lib/types';

class PageState {
	loading = $state<boolean>(true);
	saving = $state<boolean>(false);
	error = $state<string | null>(null);
	saveError = $state<string | null>(null);
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
				this.name = d.name;
			} else if (e) {
				this.error = e.description;
			}
		} finally {
			this.loading = false;
		}
	};

	handleSave = async () => {
		this.saving = true;
		this.saveError = null;

		try {
			const res = await updateSettings({
				name: this.name,
				picture: this.files[0] ?? null
			});

			const { status, error: e } = res;

			if (status === 202) {
				console.info('Profile info updated.');
				goto(routes.profile());
			} else if (e) {
				this.saveError = e.description;
			}
		} finally {
			this.saving = false;
		}
	};

	handleCancel = () => {
		goto(routes.profile());
	};

	handleFileBusy = (value: boolean) => {
		this.saving = value;
	};

	handleFileChange = (value: string[]) => {
		this.files = value;
	};

	handleNameChange = (value: string) => {
		this.name = value;
	};
}

export const pageState = new PageState();
