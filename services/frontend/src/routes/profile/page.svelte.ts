import { getMe } from '$lib/api/users';
import type { IError, IMeResponse } from '$lib/types';

class PageState {
	loading = $state<boolean>(true);
	data = $state<IMeResponse | undefined>(undefined);
	error = $state<IError | undefined>(undefined);
	statusCode = $state<number | undefined>(undefined);

	reload = async () => {
		try {
			this.loading = true;

			const { status, data: payload, error: err } = await getMe();

			this.statusCode = status;

			if (status === 200 && payload) {
				this.data = payload;
				this.error = undefined;
			} else {
				this.data = undefined;
				this.error = err;
			}
		} finally {
			this.loading = false;
		}
	};
}

export const pageState = new PageState();
