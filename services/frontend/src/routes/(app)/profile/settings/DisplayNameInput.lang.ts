import { localize } from '$lib/locale';

interface StringList {
	displayNameLabel: () => string;
}

class English implements StringList {
	displayNameLabel = () => 'Display name:';
}

class Russian extends English {
	displayNameLabel = () => 'Отображаемое имя:';
}

class Armenian extends English {
	displayNameLabel = () => 'Ցուցադրվող անուն:';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
