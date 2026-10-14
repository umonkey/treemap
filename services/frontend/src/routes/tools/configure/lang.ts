import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	name: () => string;
	description: () => string;
	apply: () => string;
	applying: () => string;
	loading: () => string;
	error: (description: string) => string;
}

class English implements StringList {
	title = () => 'Configure';
	name = () => 'Name';
	description = () => 'Description';
	apply = () => 'Apply';
	applying = () => 'Applying...';
	loading = () => 'Loading...';
	error = (description: string) => `Error: ${description}`;
}

class Russian extends English {
	title = () => 'Настройки';
	name = () => 'Название';
	description = () => 'Описание';
	apply = () => 'Применить';
	applying = () => 'Сохранение...';
	loading = () => 'Загрузка...';
	error = (description: string) => `Ошибка: ${description}`;
}

class Armenian extends English {
	title = () => 'Կարգավորումներ';
	name = () => 'Անուն';
	description = () => 'Նկարագրություն';
	apply = () => 'Կիրառել';
	applying = () => 'Պահպանվում է...';
	loading = () => 'Բեռնվում է...';
	error = (description: string) => `Սխալ: ${description}`;
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
