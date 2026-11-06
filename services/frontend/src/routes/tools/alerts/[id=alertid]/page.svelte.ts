import { getAlertFull, getAlertPhotos, type IAlertRecord } from '$lib/api/alerts';
import type { IError, IUser } from '$lib/types';
import { onPageFocus } from '$lib/utils/onPageFocus';

export class PageState {
	alert = $state<IAlertRecord | undefined>(undefined);
	users = $state<IUser[]>([]);
	photos = $state<string[]>([]);
	isLoading = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);
	currentId: string | undefined = undefined;

	reload = async (id: string, options?: { silent?: boolean }) => {
		if (this.currentId === id && !options?.silent) {
			return;
		}

		this.currentId = id;
		if (!options?.silent) {
			this.isLoading = true;
		}
		this.error = undefined;

		const [full, photos] = await Promise.all([getAlertFull(id), getAlertPhotos(id)]);

		if (!options?.silent) {
			this.isLoading = false;
		}

		if (full.status === 200 && full.data) {
			this.alert = full.data.alert;
			this.users = full.data.users;
		} else {
			this.alert = undefined;
			this.users = [];
			this.error = full.error ?? {
				code: 'load_failed',
				description: `HTTP ${full.status}`
			};
		}

		if (photos.status === 200 && photos.data) {
			this.photos = photos.data;
		} else {
			this.photos = [];
		}
	};

	reporter = (alert: IAlertRecord): IUser | undefined => {
		if (alert.source !== 'web') {
			return undefined;
		}

		return this.users.find((user) => user.id === alert.created_by);
	};

	public setup = () => {
		return onPageFocus(() => {
			if (this.currentId && !this.isLoading) {
				this.reload(this.currentId, { silent: true });
			}
		});
	};
}
