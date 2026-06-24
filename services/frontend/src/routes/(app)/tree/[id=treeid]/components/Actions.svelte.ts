import { getTreeActors } from '$lib/api/trees';
import { type IUser } from '$lib/types';

class ActionsState {
	actors = $state<IUser[]>([]);
	currentId = $state<string>('');

	public reload = async (id: string) => {
		if (this.currentId === id) return;
		this.currentId = id;

		try {
			const { status, data } = await getTreeActors(id);
			if (status === 200 && data?.users) {
				this.actors = data.users;
			} else {
				this.actors = [];
			}
		} catch (err) {
			console.error('Failed to reload tree actors:', err);
			this.actors = [];
		}
	};
}

export const actionsState = new ActionsState();
