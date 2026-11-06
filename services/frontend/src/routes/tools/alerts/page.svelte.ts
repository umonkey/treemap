import { getAlertList, type IAlertRecord } from '$lib/api/alerts';
import type { IError, IUser } from '$lib/types';
import { onPageFocus } from '$lib/utils/onPageFocus';

export class PageState {
	alerts = $state<IAlertRecord[]>([]);
	users = $state<IUser[]>([]);
	isLoading = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);

	reload = async (options?: { silent?: boolean }) => {
		if (!options?.silent) {
			this.isLoading = true;
		}
		this.error = undefined;
		const res = await getAlertList();
		if (!options?.silent) {
			this.isLoading = false;
		}
		if (res.status === 200 && res.data) {
			this.alerts = res.data.alerts;
			this.users = res.data.users;
		} else {
			this.error = res.error ?? {
				code: 'load_failed',
				description: `HTTP ${res.status}`
			};
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
			if (!this.isLoading) {
				this.reload({ silent: true });
			}
		});
	};
}
