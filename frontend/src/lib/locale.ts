class EnglishLocale {
	public appTitle(): string {
		return 'Trees of Yerevan';
	}

	public homeTitle(): string {
		return 'Tee Map';
	}

	public sideHome(): string {
		return 'Home';
	}

	public sideSearch(): string {
		return 'Search';
	}

	public sideExplore(): stirng {
		return 'Explore Map';
	}

	public sideUpdates(): stirng {
		return 'Recent updates';
	}

	public sideReports(): stirng {
		return 'Data reports';
	}

	public sideReports(): stirng {
		return 'Data reports';
	}

	public sideProfile(): stirng {
		return 'Profile';
	}

	public sideAdd(): stirng {
		return 'Add a tree';
	}

	public searchPrompt(): string {
		return 'Search trees...';
	}

	public searchLink(query: string): string {
		return `Search the map for "${query}"`;
	}

	public updatesNewTitle(): string {
		return 'New Trees';
	}

	public updatesAdded(): string {
		return 'Added';
	}

	public updatesChanged(): string {
		return 'Edited';
	}

	public updatesComments(): string {
		return 'Comments';
	}

	public addressUnknown(): string {
		return 'Unknown address';
	}

	public userUnknown(): string {
		return 'Unknown user';
	}

	public profileTrees(count: number): string {
		return `${count} trees`;
	}

	public profileUpdates(count: number): string {
		return `${count} edits`;
	}

	public profilePhotos(count: number): string {
		return `${count} photos`;
	}

	public signOut(): string {
		return 'Sign out';
	}

	public signInWithGoogle(): string {
		return 'Sign In with Google';
	}

	public profileSignInPrompt(): string {
		return 'You need to sign in to access your profile.';
	}

	public addTitle(): string {
		return 'Add Tree';
	}

	public addConfirmButton(): string {
		return 'Add tree';
	}

	public addCancelButton(): string {
		return 'Cancel';
	}

	public addConfirmLocation(): string {
		return 'Confirm location';
	}

	public speciesLabel(): string {
		return 'Species';
	}

	public speciesPrompt(): string {
		return 'Type a latin or local name';
	}

	public speciesHint(): string {
		return 'Enter the latin genus of the tree.';
	}

	public heightLabel(): string {
		return 'Height, m';
	}

	public canopyLabel(): string {
		return 'Canopy diameter, m';
	}

	public circumferenceLabel(): string {
		return 'Trunk circumference, cm';
	}

	public circumferenceHint(): string {
		return 'Measure at chest level, aka 120 cm from the ground.';
	}

	public stateLabel(): string {
		return 'State';
	}

	public stateUnknown(): string {
		return '(unknown)';
	}

	public stateHealthy(): string {
		return 'healthy';
	}

	public stateSick(): string {
		return 'sick';
	}

	public stateDeformed(): string {
		return 'deformed';
	}

	public stateDead(): string {
		return 'dead';
	}

	public stateGone(): string {
		return 'gone';
	}

	public stateStomp(): string {
		return 'stomp';
	}

	public yearLabel(): string {
		return 'Year planted';
	}

	public notesLabel(): string {
		return 'Description';
	}

	public notesHint(): string {
		return 'Add some notable details that could help visitors understand the importance of this tree.';
	}

	public detailsTitle(name: string): string {
		return `${name} — Details`;
	}

	public treeShortTitle(): string {
		return 'Tree';
	}

	public treeTabsDetails(): string {
		return 'Details';
	}

	public treeTabsMap(): string {
		return 'Map';
	}

	public treeTabsComments(): string {
		return 'Comments';
	}

	public treeTabsChanges(): string {
		return 'History';
	}

	public propHeight(): string {
		return 'Height';
	}

	public propCanopy(): string {
		return 'Canopy';
	}

	public propTrunk(): string {
		return 'Trunk';
	}

	public propState(): string {
		return 'State';
	}

	public propYear(): string {
		return 'Year';
	}

	public noDescription(): string {
		return 'There is no description for this tree. You can leave a comment to let us know about something interesting.';
	}

	public noComments(): string {
		return 'No comments for this tree yet.';
	}

	public commentPrompt(): string {
		return 'Would you like to leave a comment?';
	}

	public commentLabel(): string {
		return 'Comment';
	}

	public commentSignIn(): string {
		return 'You need to be authenticated to add comments.';
	}

	public commentSubmit(): string {
		return 'Submit comment';
	}

	public commentHint(): string {
		return 'Some hints or suggested edits.';
	}

	public contextEditTree(): string {
		return 'Edit this tree';
	}

	public contextUploadPhotos(): string {
		return 'Upload photos';
	}
}

class RussianLocale extends EnglishLocale {
	public appTitle(): string {
		return 'Деревья Еревана';
	}

	public homeTitle(): string {
		return 'Карта деревьев';
	}

	public sideHome(): string {
		return 'Главная';
	}

	public sideSearch(): string {
		return 'Поиск';
	}

	public sideExplore(): stirng {
		return 'Карта';
	}

	public sideUpdates(): stirng {
		return 'Обновления';
	}

	public sideReports(): stirng {
		return 'Анализ данных';
	}

	public sideProfile(): stirng {
		return 'Профиль';
	}

	public sideAdd(): stirng {
		return 'Добавить дерево';
	}

	public searchPrompt(): string {
		return 'Искать деревья...';
	}

	public searchLink(query: string): string {
		return `Искать "${query}" на карте`;
	}

	public updatesNewTitle(): string {
		return 'Новые деревья';
	}

	public updatesAdded(): string {
		return 'Новые';
	}

	public updatesChanged(): string {
		return 'Изменённые';
	}

	public updatesComments(): string {
		return 'Комментарии';
	}

	public addressUnknown(): string {
		return 'Адрес не указан';
	}

	public userUnknown(): string {
		return 'Пользователь неизвестен';
	}

	public profileTrees(count: number): string {
		return `${count} деревьев`;
	}

	public profileUpdates(count: number): string {
		return `${count} исправлений`;
	}

	public profilePhotos(count: number): string {
		return `${count} фотографий`;
	}

	public signOut(): string {
		return 'Выйти';
	}

	public signInWithGoogle(): string {
		return 'Войти через Google';
	}

	public profileSignInPrompt(): string {
		return 'Вам нужно залогиниться, чтобы увидеть свой профиль.';
	}

	public addTitle(): string {
		return 'Добавить дерево';
	}

	public addConfirmButton(): string {
		return 'Добавить дерево';
	}

	public addCancelButton(): string {
		return 'Отмена';
	}

	public addConfirmLocation(): string {
		return 'Подтвердите координаты';
	}

	public speciesLabel(): string {
		return 'Название вида';
	}

	public speciesPrompt(): string {
		return 'Введите латинское или местное название';
	}

	public speciesHint(): string {
		return 'Укажите латинское название вида или рода дерева.';
	}

	public heightLabel(): string {
		return 'Высота, м';
	}

	public canopyLabel(): string {
		return 'Диаметр кроны, м';
	}

	public circumferenceLabel(): string {
		return 'Обхват ствола, см';
	}

	public circumferenceHint(): string {
		return 'Измеряют на высоте 120-130 см от земли.';
	}

	public stateLabel(): string {
		return 'Состояние';
	}

	public stateUnknown(): string {
		return '(неизвестно)';
	}

	public stateHealthy(): string {
		return 'здорово';
	}

	public stateSick(): string {
		return 'больное';
	}

	public stateDeformed(): string {
		return 'деформировано';
	}

	public stateDead(): string {
		return 'мертво';
	}

	public stateGone(): string {
		return 'удалено';
	}

	public stateStomp(): string {
		return 'пень';
	}

	public yearLabel(): string {
		return 'Год посадки';
	}

	public notesLabel(): string {
		return 'Заметки';
	}

	public notesHint(): string {
		return 'Добавьте важные детали об этом дереве, которые помогут людям понять его важность.';
	}

	public detailsTitle(name: string): string {
		return `${name} — Подробности`;
	}

	public treeShortTitle(): string {
		return 'Дерево';
	}

	public treeTabsDetails(): string {
		return 'Общее';
	}

	public treeTabsMap(): string {
		return 'Карта';
	}

	public treeTabsComments(): string {
		return 'Чат';
	}

	public treeTabsChanges(): string {
		return 'Правки';
	}

	public propHeight(): string {
		return 'Высота';
	}

	public propCanopy(): string {
		return 'Крона';
	}

	public propTrunk(): string {
		return 'Обхват';
	}

	public propState(): string {
		return 'Состояние';
	}

	public propYear(): string {
		return 'Год';
	}

	public noDescription(): string {
		return 'Нет описания этого дерева. Вы можете оставить комментарий чтобы рассказать о каких-то интересных особенностях.';
	}

	public noComments(): string {
		return 'Нет комментариев для этого дерева.';
	}

	public commentPrompt(): string {
		return 'Желаете оставить комментарий?';
	}

	public commentLabel(): string {
		return 'Комментарий';
	}

	public commentSignIn(): string {
		return 'Нужно залогиниться, чтобы оставить комментарий.';
	}

	public commentSubmit(): string {
		return 'Отправить комментарий';
	}

	public commentHint(): string {
		return 'Оставьте подсказки или предложения по исправлению.';
	}

	public contextEditTree(): string {
		return 'Внести правки';
	}

	public contextUploadPhotos(): string {
		return 'Добавить фотографии';
	}
}

const guessLocale = () => {
	const lang = navigator.language || 'en-US';

	if (lang === 'ru-RU') {
		return new RussianLocale();
	}

	return new EnglishLocale();
};

export const locale = guessLocale();
