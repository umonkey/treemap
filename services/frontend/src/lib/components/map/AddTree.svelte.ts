import { goto, routes } from '$lib/routes';

class AddState {
	public toggle = (e: Event) => {
		e.preventDefault();
		goto(routes.treeAdd());
	};

	public onMount = () => {
		return () => {};
	};
}

export const addState = new AddState();
