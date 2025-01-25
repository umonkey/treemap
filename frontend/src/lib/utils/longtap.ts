const unselect = () => {
	document.getSelection().empty();
};

export const longtap = (el: HTMLElement, callback: () => void, time: number = 500): void => {
	let timer: number | null = null;

	el.addEventListener(
		'touchstart',
		() => {
			timer = setTimeout(() => {
				timer = null;
				unselect();
				callback();
			}, time);
		},
		{ passive: false }
	);

	el.addEventListener('touchend', () => {
		clearTimeout(timer);
		timer = null;
	});
};
