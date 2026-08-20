import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	titleWithCount: (count: number) => string;
	tabTitle: (query?: string) => string;
	close: () => string;
	resultsCount: (count: number) => string;
	noResults: () => string;
	loading: () => string;
	emptyQuery: () => string;
	preview: () => string;
	download: () => string;
}

class English implements StringList {
	title = () => 'Search results';
	titleWithCount = (count: number) => `Search results (${count})`;
	tabTitle = (query?: string) =>
		query ? `Search: ${query} - Trees of Yerevan` : 'Search results - Trees of Yerevan';
	close = () => 'Close';
	resultsCount = (count: number) => (count === 1 ? 'Found 1 tree' : `Found ${count} trees`);
	noResults = () => 'No trees found';
	loading = () => 'Searching...';
	emptyQuery = () => 'No search query provided';
	preview = () => 'Tree preview';
	download = () => 'Download CSV';
}

class Russian extends English {
	title = () => 'Результаты поиска';
	titleWithCount = (count: number) => `Результаты поиска (${count})`;
	tabTitle = (query?: string) =>
		query ? `Поиск: ${query} - Деревья Еревана` : 'Результаты поиска - Деревья Еревана';
	close = () => 'Закрыть';
	resultsCount = (count: number) => `Найдено деревьев: ${count}`;
	noResults = () => 'Деревья не найдены';
	loading = () => 'Поиск...';
	emptyQuery = () => 'Поисковый запрос не указан';
	preview = () => 'Просмотр дерева';
	download = () => 'Скачать CSV';
}

class Armenian extends English {
	title = () => 'Որոնման արդյունքներ';
	titleWithCount = (count: number) => `Որոնման արդյունքներ (${count})`;
	tabTitle = (query?: string) =>
		query ? `Որոնում: ${query} - Երևանի ծառերը` : 'Որոնման արդյունքներ - Երևանի ծառերը';
	close = () => 'Փակել';
	resultsCount = (count: number) => `Գտնվել է ${count} ծառ`;
	noResults = () => 'Ծառեր չեն գտնվել';
	loading = () => 'Որոնում...';
	emptyQuery = () => 'Որոնման հարցումը նշված չէ';
	preview = () => 'Ծառի դիտում';
	download = () => 'Ներբեռնել CSV';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
