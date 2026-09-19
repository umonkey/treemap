import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	downloadTitle: () => string;
	loading: () => string;
	error: (description: string) => string;
	noExports: () => string;
	file: () => string;
	size: () => string;
}

class English implements StringList {
	title = () => 'Data';
	downloadTitle = () => 'Download the database';
	loading = () => 'Loading...';
	error = (description: string) => `Error: ${description}`;
	noExports = () => 'No exports available.';
	file = () => 'File';
	size = () => 'Size';
}

class Russian extends English {
	title = () => 'Данные';
	downloadTitle = () => 'Скачать базу данных';
	loading = () => 'Загрузка...';
	error = (description: string) => `Ошибка: ${description}`;
	noExports = () => 'Нет доступных экспортов.';
	file = () => 'Файл';
	size = () => 'Размер';
}

class Armenian extends English {
	title = () => 'Տվյալներ';
	downloadTitle = () => 'Ներբեռնել տվյալների բազան';
	loading = () => 'Բեռնվում է...';
	error = (description: string) => `Սխալ: ${description}`;
	noExports = () => 'Հասանելի արտահանումներ չկան:';
	file = () => 'Ֆայլ';
	size = () => 'Չափ';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
