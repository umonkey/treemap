import type { IUser } from "$lib/types";
import { derived, writable } from "svelte/store";

type UserMap = {
	[key: string]: IUser;
};

export const userStore = writable<UserMap>({});

export const getUser = derived(userStore, ($userStore) => {
	return (id: string) => $userStore[id] ?? undefined;
});

export const addUsers = (users: IUser[]) => {
	userStore.update((store) => {
		for (const user of users) {
			store[user.id] = user;
		}

		return store;
	});
};
