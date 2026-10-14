import { localize } from '$lib/locale';

interface StringList {
	learn: () => string;
	learnDescription: () => string;
	trilaterate: () => string;
	trilaterateDescription: () => string;
	duplicates: () => string;
	duplicatesDescription: () => string;
	guessSpecies: () => string;
	guessSpeciesDescription: () => string;
	statistics: () => string;
	statisticsDescription: () => string;
	reports: () => string;
	reportsDescription: () => string;
	panoramas: () => string;
	panoramasDescription: () => string;
	users: () => string;
	usersDescription: () => string;
	exportData: () => string;
	exportDataDescription: () => string;
}

class English implements StringList {
	learn = () => 'Learn';
	learnDescription = () => 'Test your species knowledge with a random quiz and instant feedback.';
	trilaterate = () => 'Trilaterate';
	trilaterateDescription = () =>
		'Enter ground control points and distance measurements to locate a tree by trilateration.';
	duplicates = () => 'Duplicates';
	duplicatesDescription = () =>
		'Trees within a metre of each other, ready for enrichment and merge.';
	guessSpecies = () => 'Guess Species';
	guessSpeciesDescription = () =>
		'Trees with an unidentified or questionable species, ready for correction.';
	statistics = () => 'Statistics';
	statisticsDescription = () =>
		'Counts and rankings by species, status, height, trunk girth, crown diameter and address.';
	reports = () => 'Reports';
	reportsDescription = () =>
		'Pick a street to see its trees, total shade and breakdowns, with CSV export.';
	panoramas = () => 'Panoramas';
	panoramasDescription = () =>
		'Add, edit and reprocess the 360° panoramas used to photo-map trees.';
	users = () => 'Users';
	usersDescription = () => 'Browse accounts and their roles, and edit a user when needed.';
	exportData = () => 'Export';
	exportDataDescription = () => 'Download the anonymised database as a single SQLite file (ODbL).';
}

class Russian extends English {
	learn = () => 'Тренировка';
	learnDescription = () =>
		'Проверка знаний видов растений и их особенностей с помощью тренажёра.';
	trilaterate = () => 'Трилатерация';
	trilaterateDescription = () =>
		'Добавление дереьвев с помощью опорных точек и геометрии.';
	duplicates = () => 'Дубликаты';
	duplicatesDescription = () =>
		'Объединение нескольких деревьев, добавленных по ошибке, в одно.';
	guessSpecies = () => 'Определить вид';
	guessSpeciesDescription = () =>
		'Исправление данных по деревьям с неопределённым или сомнительным видом.';
	statistics = () => 'Статистика';
	statisticsDescription = () =>
		'Отчёты по видам, состоянию, высоте, обхвату ствола, диаметру кроны и адресам.';
	reports = () => 'Отчёты';
	reportsDescription = () =>
		'Отчёт по деревьям на определённой улице, выгрузка в CSV.';
	panoramas = () => 'Панорамы';
	panoramasDescription = () =>
		'Добавление и настройка уличных панорам.';
	users = () => 'Пользователи';
	usersDescription = () => 'Управление пользователями и правами доступа.';
	exportData = () => 'Выгрузка';
	exportDataDescription = () => 'Скачать копию базу одним файлом для детальных исследований SQLite (ODbL).';
}

class Armenian extends English {
	learn = () => 'Ուսուցում';
	learnDescription = () =>
		'Ստուգեք տեսակների իմացությունը պատահական թեստով և անմիջական արձագանքով։';
	trilaterate = () => 'Տրիլատերացիա';
	trilaterateDescription = () =>
		'Նշեք հենակետերը և հեռավորությունները՝ տրիլատերացիայով ծառի դիրքը որոշելու համար։';
	duplicates = () => 'Կրկնօրինակներ';
	duplicatesDescription = () =>
		'Միմյանցից մեկ մետրի սահմաններում գտնվող ծառեր՝ տվյալների լրացման և միավորման համար։';
	guessSpecies = () => 'Որոշել տեսակը';
	guessSpeciesDescription = () => 'Անհայտ կամ կասկածելի տեսակ ունեցող ծառեր՝ ճշգրտման համար։';
	statistics = () => 'Վիճակագրություն';
	statisticsDescription = () =>
		'Զեկույցներ ըստ տեսակի, վիճակի, բարձրության, բնի շրջագծի, սաղարթի տրամագծի և հասցեի։';
	reports = () => 'Զեկույցներ';
	reportsDescription = () => 'Ընտրեք փողոց՝ ծառերը, ընդհանուր ստվերն ու բաշխումը, CSV արտահանմամբ։';
	panoramas = () => 'Պանորամաներ';
	panoramasDescription = () =>
		'Ավելացրեք, խմբագրեք և վերամշակեք ծառերի քարտեզագրման 360° պանորամաները։';
	users = () => 'Օգտատերեր';
	usersDescription = () => 'Օգտատերերի և դերերի ցանկը՝ խմբագրման հնարավորությամբ։';
	exportData = () => 'Արտահանում';
	exportDataDescription = () => 'Ներբեռնեք անանունացված տվյալների բազան մեկ SQLite ֆայլով (ODbL)։';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
