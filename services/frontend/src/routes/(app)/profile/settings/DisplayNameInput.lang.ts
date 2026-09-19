import { localize } from '$lib/locale';

interface StringList {
	displayNameLabel: () => string;
	displayNameRequired: () => string;
}

class English implements StringList {
	displayNameLabel = () => 'Display name:';
	displayNameRequired = () => 'Display name cannot be empty.';
}

class Russian extends English {
	displayNameLabel = () => 'Отображаемое имя:';
	displayNameRequired = () => 'Отображаемое имя не может быть пустым.';
}

class Armenian extends English {
	displayNameLabel = () => 'Ցուցադրվող անուն:';
	displayNameRequired = () => 'Ցուցադրվող անունը չի կարող դատարկ լինել։';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
