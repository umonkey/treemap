import { verifyToken } from '$lib/api/users';
import { authStore } from '$lib/stores/authStore';
import { goto, routes } from '$lib/routes';

class PageState {
	error = $state<boolean>(false);
	loading = $state<boolean>(true);
	redirect = $state<string | null>(null);

	init = async (token: string | null, state: string | null) => {
		if (!token || !state) {
			this.error = true;
			this.loading = false;
			return;
		}

		try {
			const res = await verifyToken(token);

			if (res.status === 200 && res.data) {
				authStore.set({
					token,
					user: res.data.user,
					roles: res.data.roles || [],
					permissions: res.data.permissions || []
				});
				this.redirect = state;
			} else {
				this.error = true;
			}
		} catch (e) {
			console.error('[auth] Error during token verification', e);
			this.error = true;
		} finally {
			this.loading = false;
		}
	};

	handleRetry = () => {
		window.location.reload();
	};

	handleContinueAnonymously = () => {
		goto(routes.home());
	};
}

export const pageState = new PageState();
