export const longtap = (el: HTMLElement, callback: () => void, time: number = 500): void => {
	let timer: number | null = null;

	el.addEventListener('touchstart', (e) => {
		e.preventDefault();

		timer = setTimeout(() => {
			timer = null;
			callback();
		}, time);
	});

	el.addEventListener('touchend', () => {
		clearTimeout(timer);
		timer = null;
	});
};
