import { localize } from '$lib/locale';

interface StringList {
	edit: () => string;
	details: () => string;
	comments: () => string;
	files: () => string;
	restart: () => string;
	name: () => string;
	size: () => string;
	loading: () => string;
	noFiles: () => string;
	error: (description: string) => string;
}

class English implements StringList {
	edit = () => 'Edit';
	details = () => 'Details';
	comments = () => 'Comments';
	files = () => 'Files';
	restart = () => 'Restart';
	name = () => 'Name';
	size = () => 'Size';
	loading = () => 'Loading files…';
	noFiles = () => 'No files found.';
	error = (description: string) => `Error loading files: ${description}`;
}

class Russian extends English {
	edit = () => 'Редактировать';
	details = () => 'Детали';
	comments = () => 'Комментарии';
	files = () => 'Файлы';
	restart = () => 'Перезапустить';
	name = () => 'Имя';
	size = () => 'Размер';
	loading = () => 'Загрузка файлов…';
	noFiles = () => 'Файлы не найдены.';
	error = (description: string) => `Ошибка загрузки файлов: ${description}`;
}

class Armenian extends English {
	edit = () => 'Խմբագրել';
	details = () => 'Մանրամասներ';
	comments = () => 'Մեկնաբանություններ';
	files = () => 'Ֆայլեր';
	restart = () => 'Վերագործարկել';
	name = () => 'Անվանում';
	size = () => 'Չափ';
	loading = () => 'Ֆայլերը բեռնվում են…';
	noFiles = () => 'Ֆայլեր չեն գտնվել։';
	error = (description: string) => `Ֆայլերի բեռնման սխալ՝ ${description}`;
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
