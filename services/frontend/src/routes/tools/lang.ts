import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	heading: () => string;
	description: () => string;
}

class English implements StringList {
	title = () => 'Tools';
	heading = () => 'Additional tools';
	description = () =>
		'Here you can find various additional tools that this application provides for data management and access.';
}

class Russian extends English {
	title = () => 'Инструменты';
	heading = () => 'Инструменты';
	description = () =>
		'Здесь вы найдёте различные дополнительные инструменты, которые приложение предоставляет для управления данными и доступа к ним.';
}

class Armenian extends English {
	title = () => 'Գործիքներ';
	heading = () => 'Լրացուցիչ գործիքներ';
	description = () =>
		'Այստեղ դուք կգտնեք տարբեր լրացուցիչ գործիքներ, որոնք հավելվածը տրամադրում է տվյալների կառավարման և հասանելիության համար:';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
