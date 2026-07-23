import { localize } from '$lib/locale';

interface StringList {
	authTitle: () => string;
	authError: () => string;
	authTryAgain: () => string;
	authContinueAnonymously: () => string;
}

class English implements StringList {
	authTitle = () => 'Authentication';
	authError = () => 'Could not authenticate you, please try again in a while.';
	authTryAgain = () => 'Try again';
	authContinueAnonymously = () => 'Continue anonymously';
}

class Russian extends English {
	authTitle = () => 'Аутентификация';
	authError = () => 'Не удалось войти в систему, пожалуйста, попробуйте позже.';
	authTryAgain = () => 'Попробовать снова';
	authContinueAnonymously = () => 'Продолжить анонимно';
}

class Armenian extends English {
	authTitle = () => 'Նույնականացում';
	authError = () => 'Հնարավոր չէր նույնականացնել Ձեզ, խնդրում ենք փորձել մի փոքր ուշ:';
	authTryAgain = () => 'Փորձել կրկին';
	authContinueAnonymously = () => 'Շարունակել անանուն';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
