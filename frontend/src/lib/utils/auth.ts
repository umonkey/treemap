import { apiClient } from "$lib/api";
import { authStore } from "$lib/stores/authStore";
import { toast } from "@zerodevx/svelte-toast";
import { get } from "svelte/store";

type LoginData = {
	credential: string;
};

export const googleCallbackHandler = async (user: LoginData) => {
	const token = user.credential;

	const res = await apiClient.loginWithGoogle(token);

	if (res.status === 200 && res.data) {
		authStore.set(res.data);
		console.info(`Logged in as ${res.data.name}`);
	} else {
		toast.push(`Error ${res.status} getting an authentication token.`);
	}
};

export const validateStoredToken = async () => {
	const auth = get(authStore);

	if (auth === undefined) {
		console.debug("[auth] Not authenticated.");
		return;
	}

	if (auth.token === undefined) {
		console.debug("[auth] No auth token stored.");
		authStore.update(() => undefined);
		return;
	}

	console.debug("[auth] Validating stored auth token...", auth.token);

	const res = await apiClient.verifyToken(auth.token);

	if (res.status === 401) {
		console.info("[auth] Token expired.");
		authStore.update(() => undefined);
		return;
	}

	console.debug("[auth] Token is OK.");
};
