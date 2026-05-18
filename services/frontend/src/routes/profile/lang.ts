import { localize } from '$lib/locale';

interface StringList {
	profileTitle: () => string;
	uploadsTitle: () => string;
	settingsTitle: () => string;
}

class English implements StringList {
	profileTitle = () => 'Profile';
	uploadsTitle = () => 'Uploads';
	settingsTitle = () => 'Settings';
}

class Russian extends English {
	profileTitle = () => 'Профиль';
	uploadsTitle = () => 'Загрузки';
	settingsTitle = () => 'Настройки';
}

class Armenian extends English {
	profileTitle = () => 'Պրոֆիլ';
	uploadsTitle = () => 'Վերբեռնումներ';
	settingsTitle = () => 'Կարգավորումներ';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
