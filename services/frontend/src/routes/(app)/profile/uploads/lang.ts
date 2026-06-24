import { localize } from '$lib/locale';

interface StringList {
	profileTitle: () => string;
	uploadsTitle: () => string;
	uploadsDisabledHint: () => string;
	uploadsEnabledHint: () => string;
	uploadsEmpty: () => string;
	uploadsStart: () => string;
	uploadsRestart: () => string;
}

class English implements StringList {
	profileTitle = () => 'Profile';
	uploadsTitle = () => 'Uploads';
	uploadsDisabledHint = () =>
		'Files will not be uploading unless you open this page and click the start button. You can queue the files and upload them when you feel comfortable.';
	uploadsEnabledHint = () =>
		'The photos will be uploading as soon as you take them, unless your phone is in data saving or power saving mode.';
	uploadsEmpty = () =>
		'There are no pending file uploads. Please take some photos and they will show up here.';
	uploadsStart = () => 'Start uploads';
	uploadsRestart = () => 'Restart failed';
}

class Russian extends English {
	profileTitle = () => 'Профиль';
	uploadsTitle = () => 'Загрузки';
	uploadsDisabledHint = () =>
		'Файлы не будут загружаться, пока вы не откроете эту страницу и не нажмете кнопку запуска. Вы можете ставить файлы в очередь и загружать их, когда вам будет удобно.';
	uploadsEnabledHint = () =>
		'Фотографии будут загружаться сразу после того, как вы их сделаете, если только ваш телефон не находится в режиме экономии трафика или энергии.';
	uploadsEmpty = () =>
		'Нет ожидающих загрузок. Сделайте несколько фотографий, и они появятся здесь.';
	uploadsStart = () => 'Загрузить';
	uploadsRestart = () => 'Повторить';
}

class Armenian extends English {
	uploadsDisabledHint = () =>
		'Ֆայլերը չեն վերբեռնվի, քանի դեռ չեք բացել այս էջը և սեղմել սկսելու կոճակը: Դուք կարող եք հերթագրել ֆայլերը և վերբեռնել դրանք, երբ ձեզ հարմար լինի:';
	uploadsEnabledHint = () =>
		'Լուսանկարները կվերբեռնվեն հենց որ դրանք նկարեք, եթե ձեր հեռախոսը տվյալների խնայողության կամ էներգիայի խնայողության ռեժիմում չէ:';
	uploadsEmpty = () => 'Վերբեռնվող ֆայլեր չկան: Լուսանկարեք, և դրանք կհայտնվեն այստեղ:';
	uploadsStart = () => 'Վերբեռնել';
	uploadsRestart = () => 'Վերսկսել';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
