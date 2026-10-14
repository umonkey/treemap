import { localize } from '$lib/locale';

interface StringList {
	noRecentContributions: () => string;
}

class English implements StringList {
	noRecentContributions = () => 'No contributions in the last 30 days.';
}

class Russian extends English {
	noRecentContributions = () => 'Нет вклада за последние 30 дней.';
}

class Armenian extends English {
	noRecentContributions = () => 'Վերջին 30 օրվա ընթացքում ներդրումներ չկան։';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
