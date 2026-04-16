import { goto, routes } from '$lib/routes';

export const handleClose = async (e?: Event) => {
	e?.preventDefault();
	await goto(routes.home());
};
