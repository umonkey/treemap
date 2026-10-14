import { localize } from '$lib/locale';

interface StringList {
	learn: () => string;
	trilaterate: () => string;
	duplicates: () => string;
	guessSpecies: () => string;
	statistics: () => string;
	reports: () => string;
	panoramas: () => string;
	users: () => string;
	exportData: () => string;
}

class English implements StringList {
	learn = () => 'Learn';
	trilaterate = () => 'Trilaterate';
	duplicates = () => 'Duplicates';
	guessSpecies = () => 'Guess Species';
	statistics = () => 'Statistics';
	reports = () => 'Reports';
	panoramas = () => 'Panoramas';
	users = () => 'Users';
	exportData = () => 'Export';
}

class Russian extends English {
	learn = () => 'Тренировка';
	trilaterate = () => 'Трилатерация';
	duplicates = () => 'Дубликаты';
	guessSpecies = () => 'Определить вид';
	statistics = () => 'Статистика';
	reports = () => 'Отчёты';
	panoramas = () => 'Панорамы';
	users = () => 'Пользователи';
	exportData = () => 'Выгрузка';
}

class Armenian extends English {
	learn = () => 'Ուսուցում';
	trilaterate = () => 'Տրիլատերացիա';
	duplicates = () => 'Կրկնօրինակներ';
	guessSpecies = () => 'Որոշել տեսակը';
	statistics = () => 'Վիճակագրություն';
	reports = () => 'Զեկույցներ';
	panoramas = () => 'Պանորամաներ';
	users = () => 'Օգտատերեր';
	exportData = () => 'Արտահանում';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
