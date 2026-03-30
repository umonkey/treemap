export const handleClose = async (e?: Event) => {
	e?.preventDefault();
	history.back();
};
