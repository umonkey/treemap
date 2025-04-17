const unselect = () => {
	document.getSelection()?.empty();
};

export const longtap = (
	el: HTMLElement,
	callback: () => void,
	time = 500,
): void => {
	let timer: number | null = null;

	el.addEventListener(
		"touchstart",
		() => {
			timer = window.setTimeout(() => {
				timer = null;
				unselect();
				callback();
			}, time);
		},
		{ passive: false },
	);

	el.addEventListener("touchend", () => {
		if (timer) {
			window.clearTimeout(timer);
			timer = null;
		}
	});
};
