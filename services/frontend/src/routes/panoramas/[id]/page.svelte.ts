import { goto, routes } from '$lib/routes';

class PageState {
	id = $state<string>('');

	public handleClose = async () => {
		await goto(routes.home());
	};

	public reload = (id: string) => {
		this.id = id;
	};
}

export const pageState = new PageState();
