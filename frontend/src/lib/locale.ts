class EnglishLocale {
	public readonly lang: string = 'en';

	public appTitle(): string {
		return 'Trees of Yerevan';
	}

	public homeTitle(): string {
		return 'Tree Map';
	}

	public sideHome(): string {
		return 'Home';
	}

	public sideSearch(): string {
		return 'Search';
	}

	public sideExplore(): string {
		return 'Explore Map';
	}

	public sideUpdates(): string {
		return 'Recent updates';
	}

	public sideReports(): string {
		return 'Data reports';
	}

	public sideProfile(): string {
		return 'Profile';
	}

	public profileTitle(): string {
		return 'Profile';
	}

	public sideAdd(): string {
		return 'Add a tree';
	}

	public sideAbout(): string {
		return 'About this app';
	}

	public sideBugs(): string {
		return 'Bugs';
	}

	public searchTitle(): string {
		return 'Search';
	}

	public reportTitle(): string {
		return 'Street Report';
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

	public updatesComments(): string {
		return 'Comments';
	}

	public addressLabel(): string {
		return 'Address';
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

	public addRowTitle(): string {
		return 'Add a row of trees';
	}

	public addConfirmButton(): string {
		return 'Add tree';
	}

	public addRowConfirmButton(): string {
		return 'Add trees';
	}

	public addCancelButton(): string {
		return 'Cancel';
	}

	public addConfirmLocation(): string {
		return 'Confirm location';
	}

	public editTitle(): string {
		return 'Edit Tree';
	}

	public speciesLabel(): string {
		return 'Species';
	}

	public streetLabel(): string {
		return 'Street';
	}

	public speciesPrompt(): string {
		return 'Type a latin or local name';
	}

	public speciesHint(): string {
		return 'Enter the latin genus of the tree.';
	}

	public streetHint(): string {
		return 'Enter the English name of the street.';
	}

	public heightLabel(): string {
		return 'Height, m';
	}

	public rowSizeLabel(): string {
		return 'Trees in the row';
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
		return 'alive';
	}

	public stateDead(): string {
		return 'dead';
	}

	public stateGone(): string {
		return 'gone';
	}

	public stateStump(): string {
		return 'stump';
	}

	public stateReplaced(): string {
		return 'replaced';
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

	public treeTabsStreetView(): string {
		return '360°';
	}

	public treeTabsMap(): string {
		return 'Map';
	}

	public treeTabsObservations(): string {
		return 'Observations';
	}

	public observationBarkDamage(): string {
		return 'Bark damage';
	}
	public observationDryBranches(): string {
		return 'Dry branches';
	}
	public observationLeaking(): string {
		return 'Leaking';
	}
	public observationRootDamage(): string {
		return 'Root damage';
	}
	public observationOpenRoots(): string {
		return 'Open roots';
	}
	public observationBugHoles(): string {
		return 'Bug holes';
	}

	public observationTopping(): string {
		return 'Topping';
	}
	public observationFungalBodies(): string {
		return 'Fungal bodies';
	}
	public observationVfork(): string {
		return 'V-fork';
	}
	public observationCavities(): string {
		return 'Cavities';
	}
	public observationVines(): string {
		return 'Vines';
	}
	public observationInclined(): string {
		return 'Inclined';
	}
	public observationNests(): string {
		return 'Nests';
	}
	public observationNestingBoxes(): string {
		return 'Nesting boxes';
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

	public observationSignIn(): string {
		return 'You need to be authenticated to save observations.';
	}

	public commentSubmit(): string {
		return 'Submit comment';
	}

	public commentHint(): string {
		return 'Some hints or suggested edits.';
	}

	public contextReplace(): string {
		return 'Replace tree';
	}

	public contextEditTree(): string {
		return 'Other changes';
	}

	public contextUploadPhotos(): string {
		return 'Upload photos';
	}

	public contextMeasure(): string {
		return 'Measures';
	}

	public contextDead(): string {
		return 'Tree is dead';
	}

	public contextGone(): string {
		return 'Tree is gone';
	}

	public contextMove(): string {
		return 'Update location';
	}

	public contextHeight(): string {
		return 'Update height';
	}

	public contextDiameter(): string {
		return 'Update crown';
	}

	public contextCircumference(): string {
		return 'Update circumference';
	}

	public locationLabel(): string {
		return 'Location';
	}

	public editSave(): string {
		return 'Save changes';
	}

	public editCancel(): string {
		return 'Cancel';
	}

	public mapTitle(): string {
		return 'Map';
	}

	public mapTitleQuery(query: string): string {
		return `Map: ${query}`;
	}

	public learnTitle(): string {
		return 'Training';
	}

	public sideLearn(): string {
		return 'Training';
	}

	public learnDone(): string {
		return 'Done!';
	}

	public learnScore(correct: number, total: number): string {
		return `Your score is: ${correct} of ${total}`;
	}

	public learnRetry(): string {
		return 'Try again';
	}

	public learnPerfect(): string {
		return 'Perfect!';
	}

	public learnGreat(): string {
		return 'Great!';
	}

	public learnGood(): string {
		return 'Needs more work';
	}

	public learnBad(): string {
		return 'Needs a lot more work!';
	}

	public learnCorrect(): string {
		return 'Correct!';
	}

	public learnConfirm(): string {
		return 'Confirm';
	}

	public learnWrong(): string {
		return 'Wrong!';
	}

	public learnCorrectAnswer(value: string): string {
		return `Correct answer: ${value}`;
	}

	public learnContinue(): string {
		return 'Continue';
	}

	public helpCircumference(): string {
		return 'https://myga.am/app/measuring-circumference.html';
	}

	public photoTake(): string {
		return 'Take a photo';
	}

	public photoUpload(): string {
		return 'Upload files';
	}

	public photoSelect(): string {
		return 'Select files';
	}

	public photoIntro(): string {
		return 'Please click the camera icon to take all the photos, then upload them.';
	}

	public photoTitle(): string {
		return 'Upload photos';
	}

	public deleteTreeTitle(): string {
		return 'Delete tree';
	}

	public deleteConfirm(): string {
		return 'Confirm removal';
	}

	public deleteHeader(): string {
		return 'The tree is now gone, is this correct?';
	}

	public deleteUploadHint(): string {
		return 'Please also upload a photo confirming that the tree is gone.';
	}

	public deleteCommentHint(): string {
		return 'Optional comment, e.g.: duplicate tree, added by error.';
	}

	public deadTitle(): string {
		return 'Dead tree';
	}

	public deadHeader(): string {
		return 'The tree is completely dead, is this correct?';
	}

	public deadUploadHint(): string {
		return 'Please also upload a photo confirming that the tree is now dead.';
	}

	public deadConfirm(): string {
		return 'Confirm death';
	}

	public measureTitle(): string {
		return 'Measure tree';
	}

	public replaceTitle(): string {
		return 'Replace tree';
	}

	public meters(value: string): string {
		return `${value} m`;
	}

	public centimeters(value: string): string {
		return `${value} cm`;
	}

	public chooseFocus(): string {
		return 'Please choose your focus';
	}

	public enterMapperMode(): string {
		return 'Enter Tree Mapper';
	}

	public enterExplorerMode(): string {
		return 'Enter Tree Explorer';
	}

	public deleteNotification(): string {
		return 'Tree deleted.';
	}

	public noChangeHistory(): string {
		return 'No changes found for this tree.';
	}

	public replaceHint(): string {
		return 'You are about to replace a tree with a new one. The old tree will be marked as gone, the new one will be added at the exact location. The trees will be linekd to track planting history.';
	}

	public photosAdded(): string {
		return 'Finished uploading all photos. They will show shortly.';
	}

	public settingsButton(): string {
		return 'Settings';
	}

	public settingsTitle(): string {
		return 'Settings';
	}

	public mapPreviewDetails(): string {
		return 'Details';
	}

	public rowStepInfo(count: number, step: number): string {
		return `You will add ${count} trees, spaced ${step.toFixed(1)} meters apart.`;
	}

	public rowLength(value: number): string {
		return `Length: ${value.toFixed(1)} m`;
	}

	public addRowIntro(): string {
		return 'Move the map to set the end of the row.';
	}

	public addContinueButton(): string {
		return 'Continue';
	}

	public globalHeatmapHeader(): string {
		return 'Recent activity';
	}

	public learnContributions(): string {
		return 'Learn how we count contributions';
	}

	public uploadsTitle(): string {
		return 'Uploads';
	}

	public uploadsDate(): string {
		return 'Date';
	}

	public uploadsStatus(): string {
		return 'Status';
	}

	public uploadsSize(): string {
		return 'Size';
	}

	public uploadsAuto(): string {
		return 'Auto-upload files';
	}

	public toastErrorAddingComment(): string {
		return 'Error adding comment.';
	}

	public shortMonday(): string {
		return 'Mon';
	}

	public shortWednesday(): string {
		return 'Wed';
	}

	public shortFriday(): string {
		return 'Fri';
	}

	public shortSunday(): string {
		return 'Sun';
	}

	public tabTrees(): string {
		return 'Trees';
	}
}

class RussianLocale extends EnglishLocale {
	public override readonly lang = 'ru';

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

	public sideExplore(): string {
		return 'Карта';
	}

	public sideUpdates(): string {
		return 'Обновления';
	}

	public sideReports(): string {
		return 'Анализ данных';
	}

	public sideProfile(): string {
		return 'Профиль';
	}

	public profileTitle(): string {
		return 'Профиль';
	}

	public sideAdd(): string {
		return 'Добавить дерево';
	}

	public sideAbout(): string {
		return 'О нас';
	}

	public sideBugs(): string {
		return 'Баги';
	}

	public searchTitle(): string {
		return 'Поиск';
	}

	public reportTitle(): string {
		return 'Отчёт по улице';
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

	public addRowTitle(): string {
		return 'Добавить ряд деревьев';
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

	public streetLabel(): string {
		return 'Улица';
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
		return 'живое';
	}

	public stateDead(): string {
		return 'мертво';
	}

	public stateGone(): string {
		return 'удалено';
	}

	public stateStump(): string {
		return 'пень';
	}

	public stateReplaced(): string {
		return 'заменено';
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

	public treeTabsObservations(): string {
		return 'Наблюдения';
	}

	public observationBarkDamage(): string {
		return 'Повреждение коры';
	}
	public observationDryBranches(): string {
		return 'Сухие ветки';
	}
	public observationLeaking(): string {
		return 'Течёт жидкость';
	}
	public observationRootDamage(): string {
		return 'Повреждение корней';
	}
	public observationOpenRoots(): string {
		return 'Открытые корни';
	}
	public observationBugHoles(): string {
		return 'Отверстия от насекомых';
	}

	public observationTopping(): string {
		return 'Топпинг';
	}
	public observationFungalBodies(): string {
		return 'Грибы (плодовые тела)';
	}
	public observationVfork(): string {
		return 'V-образная развилка';
	}
	public observationCavities(): string {
		return 'Дупла';
	}
	public observationVines(): string {
		return 'Лианы';
	}
	public observationInclined(): string {
		return 'Наклон';
	}
	public observationNests(): string {
		return 'Гнёзда';
	}
	public observationNestingBoxes(): string {
		return 'Скворечники';
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

	public observationSignIn(): string {
		return 'Нужно залогиниться, чтобы сохранить наблюдения.';
	}

	public commentSubmit(): string {
		return 'Отправить комментарий';
	}

	public commentHint(): string {
		return 'Оставьте подсказки или предложения по исправлению.';
	}

	public contextReplace(): string {
		return 'Заменить дерево';
	}

	public contextEditTree(): string {
		return 'Внести другие правки';
	}

	public contextUploadPhotos(): string {
		return 'Добавить фотографии';
	}

	public contextMeasure(): string {
		return 'Замеры';
	}

	public contextDead(): string {
		return 'Дерево засохло';
	}

	public contextGone(): string {
		return 'Дерево убрано';
	}

	public editTitle(): string {
		return 'Редактор дерева';
	}

	public locationLabel(): string {
		return 'Координаты';
	}

	public editSave(): string {
		return 'Сохранить изменения';
	}

	public editCancel(): string {
		return 'Отмена';
	}

	public mapTitle(): string {
		return 'Карта';
	}

	public mapTitleQuery(query: string): string {
		return `Карта: ${query}`;
	}

	public learnTitle(): string {
		return 'Тренировка';
	}

	public sideLearn(): string {
		return 'Тренировка';
	}

	public learnScore(correct: number, total: number): string {
		return `Твой результат: ${correct} из ${total}`;
	}

	public learnRetry(): string {
		return 'Попробовать ещё раз';
	}

	public learnPerfect(): string {
		return 'Идеально!';
	}

	public learnGreat(): string {
		return 'Отлично!';
	}

	public learnGood(): string {
		return 'Неплохо';
	}

	public learnBad(): string {
		return 'Нужно больше тренировок';
	}

	public learnCorrect(): string {
		return 'Верно!';
	}

	public learnConfirm(): string {
		return 'Проверить';
	}

	public learnWrong(): string {
		return 'Ошибка!';
	}

	public learnCorrectAnswer(value: string): string {
		return `Правильный ответ: ${value}`;
	}

	public learnContinue(): string {
		return 'Продолжить';
	}

	public helpCircumference(): string {
		return 'https://myga.am/ru/app/measuring-circumference.html';
	}

	public photoTake(): string {
		return 'Сделать фото';
	}

	public photoUpload(): string {
		return 'Загрузить файлы';
	}

	public photoSelect(): string {
		return 'Выбрать файлы';
	}

	public photoIntro(): string {
		return 'Используй значок камеры, чтобы сделать нужные фотографии, потом загрузи их.';
	}

	public photoTitle(): string {
		return 'Загрузка фотографий';
	}

	public measureTitle(): string {
		return 'Замер дерева';
	}

	public replaceTitle(): string {
		return 'Замена дерева';
	}

	public meters(value: string): string {
		return `${value} м`;
	}

	public centimeters(value: string): string {
		return `${value} см`;
	}

	public chooseFocus(): string {
		return 'Выберите фокус';
	}

	public enterMapperMode(): string {
		return 'Войти в режим маппера';
	}

	public enterExplorerMode(): string {
		return 'Войти в режим исследователя';
	}

	public deleteTreeTitle(): string {
		return 'Удалить дерево';
	}

	public deleteHeader(): string {
		return 'Это дерево было удалено, всё верно?';
	}

	public deleteConfirm(): string {
		return 'Подтвердить удаление';
	}

	public deleteNotification(): string {
		return 'Дерево удалено';
	}

	public addressLabel(): string {
		return 'Адрес';
	}

	public deleteUploadHint(): string {
		return 'Пожалуйста, также загрузите фото, подтверждающее, что дерево убрано.';
	}

	public deadTitle(): string {
		return 'Мёртвое дерево';
	}

	public deadHeader(): string {
		return 'Это дерево мёртвое, всё верно?';
	}

	public deadUploadHint(): string {
		return 'Пожалуйста, также загрузите фото, подтверждающее, что дерево засохло.';
	}

	public deadConfirm(): string {
		return 'Подтвердить';
	}

	public contextHeight(): string {
		return 'Обновить высоту';
	}

	public contextDiameter(): string {
		return 'Обновить крону';
	}

	public contextCircumference(): string {
		return 'Обновить обхват ствола';
	}

	public noChangeHistory(): string {
		return 'Нет изменений для этого дерева.';
	}

	public contextMove(): string {
		return 'Уточнить координаты';
	}

	public replaceHint(): string {
		return 'Вы собираетесь заменить дерево на новое. Старое дерево будет помечено как удалённое, новое будет добавлено в том же месте. Деревья будут связаны, чтобы отслеживать историю посадки.';
	}

	public photosAdded(): string {
		return 'Загрузка завершена. Фотографии скоро появятся.';
	}

	public settingsButton(): string {
		return 'Настройки';
	}

	public settingsTitle(): string {
		return 'Настройки';
	}

	public mapPreviewDetails(): string {
		return 'Детали';
	}

	public rowSizeLabel(): string {
		return 'Количество деревьев';
	}

	public rowStepInfo(count: number, step: number): string {
		return `Добавим ${count} деревьев, с шагом ${step.toFixed(1)} метров.`;
	}

	public addRowConfirmButton(): string {
		return 'Добавить ряд';
	}

	public rowLength(value: number): string {
		return `Длина: ${value.toFixed(1)} м`;
	}

	public addContinueButton(): string {
		return 'Продолжить';
	}

	public addRowIntro(): string {
		return 'Двигай карту чтобы указать конец ряда.';
	}

	public globalHeatmapHeader(): string {
		return 'Недавняя активность';
	}

	public learnContributions(): string {
		return 'Узнать как мы это считаем';
	}

	public uploadsTitle(): string {
		return 'Загрузки';
	}

	public uploadsDate(): string {
		return 'Дата';
	}

	public uploadsStatus(): string {
		return 'Статус';
	}

	public uploadsSize(): string {
		return 'Размер';
	}

	public uploadsAuto(): string {
		return 'Загружать файлы автоматически';
	}

	public toastErrorAddingComment(): string {
		return 'Ошибка добавления комментария.';
	}

	public shortMonday(): string {
		return 'Пн';
	}

	public shortWednesday(): string {
		return 'Ср';
	}

	public shortFriday(): string {
		return 'Пт';
	}

	public shortSunday(): string {
		return 'Вс';
	}

	public tabTrees(): string {
		return 'Деревья';
	}
}

class ArmenianLocale extends EnglishLocale {
	public override readonly lang = 'hy';

	public appTitle(): string {
		return 'Երևանի ծառեր';
	}

	public homeTitle(): string {
		return 'Ծառերի քարտեզ';
	}

	public sideHome(): string {
		return 'Գլխավոր';
	}

	public sideSearch(): string {
		return 'Որոնել';
	}

	public sideExplore(): string {
		return 'Տեսնել քարտեզը';
	}

	public sideUpdates(): string {
		return 'Վերջին թարմացումներ';
	}

	public sideReports(): string {
		return 'Տվյալների զեկույցներ';
	}

	public sideProfile(): string {
		return 'Պրոֆիլ';
	}

	public profileTitle(): string {
		return 'Պրոֆիլ';
	}

	public sideAdd(): string {
		return 'Ավելացնել ծառ';
	}

	public sideAbout(): string {
		return 'Այս ծրագրի մասին';
	}

	public searchTitle(): string {
		return 'Որոնել';
	}

	public searchPrompt(): string {
		return 'Գտնել ծառեր...';
	}

	public searchLink(query: string): string {
		return `Որոնել քարտեզում "${query}"`;
	}

	public updatesNewTitle(): string {
		return 'Նոր ծառեր';
	}

	public updatesAdded(): string {
		return 'Ավելացված';
	}

	public updatesComments(): string {
		return 'Մեկնաբանություններ';
	}

	public addressUnknown(): string {
		return 'Անհայտ հասցե';
	}

	public userUnknown(): string {
		return 'Անհայտ օգտատեր';
	}

	public profileTrees(count: number): string {
		return `${count} ծառ`;
	}

	public profileUpdates(count: number): string {
		return `${count} խմբագրում`;
	}

	public profilePhotos(count: number): string {
		return `${count} լուսանկար`;
	}

	public signOut(): string {
		return 'Ելք';
	}

	public signInWithGoogle(): string {
		return 'Մուտք գործել Google-ով';
	}

	public profileSignInPrompt(): string {
		return 'Դուք պետք է մուտք գործեք Ձեր պրոֆիլին հասնաելիություն ստանալու համար:';
	}

	public addTitle(): string {
		return 'Ավելացնել ծառ';
	}

	public addRowTitle(): string {
		return 'Ավելացրե՛ք ծառերի մի շարք';
	}

	public addConfirmButton(): string {
		return 'Ավելացնել ծառ';
	}

	public addCancelButton(): string {
		return 'Չեղարկել';
	}

	public addConfirmLocation(): string {
		return 'Հաստատել տեղադրությունը';
	}

	public editTitle(): string {
		return 'Խմբագրել ծառը';
	}

	public speciesLabel(): string {
		return 'Տեսակ';
	}

	public streetLabel(): string {
		return 'Փողոց';
	}

	public speciesPrompt(): string {
		return 'Մուտքագրեք լատիներեն կամ հայերեն անուն';
	}

	public speciesHint(): string {
		return 'Մուտքագրեք ծառի լատիներեն սեռը:';
	}

	public heightLabel(): string {
		return 'Բարձրություն, մ';
	}

	public canopyLabel(): string {
		return 'Հովանի տրամագիծ, մ';
	}

	public circumferenceLabel(): string {
		return 'Բնի շրջագիծ, սմ';
	}

	public circumferenceHint(): string {
		return 'Չափել կրծքավանդակի մակարդակով, այսինքն՝ 120 սմ հողից:';
	}

	public stateLabel(): string {
		return 'Վիճակը';
	}

	public stateUnknown(): string {
		return '(անհայտ)';
	}

	public stateHealthy(): string {
		return 'կենդանի';
	}

	public stateDead(): string {
		return 'չորացած';
	}

	public stateGone(): string {
		return 'բացակայում է';
	}

	public stateStump(): string {
		return 'կոճղ';
	}

	public stateReplaced(): string {
		return 'Փոխարինված';
	}

	public yearLabel(): string {
		return 'Տնկման տարի';
	}

	public notesLabel(): string {
		return 'Նկարագրություն';
	}

	public notesHint(): string {
		return 'Ավելացրեք որոշ նշանակալի մանրամասներ, որոնք կարող են օգնել այցելուներին հասկանալ այս ծառի կարևորությունը:';
	}

	public detailsTitle(name: string): string {
		return `${name} — Մանրամասներ`;
	}

	public treeShortTitle(): string {
		return 'Ծառ';
	}

	public treeTabsDetails(): string {
		return 'Մանրամասներ';
	}

	public treeTabsMap(): string {
		return 'Քարտեզ';
	}

	public treeTabsObservations(): string {
		return 'Դիտարկումներ';
	}

	public observationBarkDamage(): string {
		return 'Կեղևի վնասվածք';
	}
	public observationDryBranches(): string {
		return 'Չոր ճյուղեր';
	}
	public observationLeaking(): string {
		return 'Հյութահոսություն';
	}
	public observationRootDamage(): string {
		return 'Արմատների վնասվածք';
	}
	public observationOpenRoots(): string {
		return 'Բաց արմատներ';
	}
	public observationBugHoles(): string {
		return 'Միջատների անցքեր';
	}

	public observationTopping(): string {
		return 'Գագատահատում';
	}
	public observationFungalBodies(): string {
		return 'Սնկեր';
	}
	public observationVfork(): string {
		return 'V-աձև ճյուղավորում';
	}
	public observationCavities(): string {
		return 'Փչակներ';
	}
	public observationVines(): string {
		return 'Մագլցող բույսեր';
	}
	public observationInclined(): string {
		return 'Թեքված';
	}
	public observationNests(): string {
		return 'Բներ';
	}
	public observationNestingBoxes(): string {
		return 'Թռչնաբներ';
	}

	public treeTabsChanges(): string {
		return 'Պատմություն';
	}

	public propHeight(): string {
		return 'Բարձրություն';
	}

	public propCanopy(): string {
		return 'Հովանի';
	}

	public propTrunk(): string {
		return 'Բուն';
	}

	public propState(): string {
		return 'Վիճակ';
	}

	public propYear(): string {
		return 'Տարի';
	}

	public noComments(): string {
		return 'Այս ծառի մասին դեռևս մեկնաբանություններ չկան:';
	}

	public commentPrompt(): string {
		return 'Կցանկանայի՞ք մեկնաբանություն թողնել:';
	}

	public commentLabel(): string {
		return 'Մեկնաբանություն';
	}

	public commentSignIn(): string {
		return 'Մեկնաբանություններ ավելացնելու համար անհրաժեշտ է վավերացում:';
	}

	public observationSignIn(): string {
		return 'Դիտարկումները պահպանելու համար անհրաժեշտ է վավերացում:';
	}

	public commentSubmit(): string {
		return 'Հաստատել մեկնաբանությունը';
	}

	public commentHint(): string {
		return 'Որոշ խորհուրդներ կամ առաջարկվող խմբագրումներ:';
	}

	public contextEditTree(): string {
		return 'Խմբագրել այս ծառը';
	}

	public contextUploadPhotos(): string {
		return 'Վերբեռնել լուսանկարներ';
	}

	public contextMeasure(): string {
		return 'Չափումներ';
	}

	public locationLabel(): string {
		return 'Տեղադրություն';
	}

	public editSave(): string {
		return 'Պահպանել փոփոխությունները';
	}

	public editCancel(): string {
		return 'Չեղարկել';
	}

	public mapTitle(): string {
		return 'Քարտեզ';
	}

	public mapTitleQuery(query: string): string {
		return `Քարտեզ: ${query}`;
	}

	public learnTitle(): string {
		return 'Ուսուցում';
	}

	public sideLearn(): string {
		return 'Ուսուցում';
	}

	public learnDone(): string {
		return 'Ավարտված է!';
	}

	public learnScore(correct: number, total: number): string {
		return `Ձեր արդյունքը՝ ${correct}-ը ${total}-ից`;
	}

	public learnRetry(): string {
		return 'Կրկին փորձել';
	}

	public learnPerfect(): string {
		return 'Հիանալի!';
	}

	public learnGreat(): string {
		return 'Հիանալի!';
	}

	public learnGood(): string {
		return 'Պահանջվում է ավելի շատ աշխատանք';
	}

	public learnBad(): string {
		return 'Պահանջվում է շատ ավելի աշխատանք!';
	}

	public learnCorrect(): string {
		return 'Ճիշտ!';
	}

	public learnConfirm(): string {
		return 'Հաստատել';
	}

	public learnWrong(): string {
		return 'Սխալ!';
	}

	public learnCorrectAnswer(value: string): string {
		return `Ճիշտ պատասխան՝ ${value}`;
	}

	public learnContinue(): string {
		return 'Շարունակել';
	}

	public helpCircumference(): string {
		return 'https://myga.am/app/measuring-circumference.html';
	}

	public photoTake(): string {
		return 'Լուսանկարել';
	}

	public photoUpload(): string {
		return 'Վերբեռնել ֆայլեր';
	}

	public photoSelect(): string {
		return 'Ընտրել ֆայլեր';
	}

	public photoIntro(): string {
		return 'Խնդրում ենք սեղմել տեսախցիկի պատկերակին, նկարեք բոլոր լուսանկարները, այնուհետև վերբեռնեք դրանք։';
	}

	public photoTitle(): string {
		return 'Վերբեռնել լուսանկարներ';
	}

	public measureTitle(): string {
		return 'Չափել ծառը';
	}

	public meters(value: string): string {
		return `${value} մ`;
	}

	public centimeters(value: string): string {
		return `${value} սմ`;
	}

	public settingsButton(): string {
		return 'Կարգեր';
	}

	public settingsTitle(): string {
		return 'Կարգավորումներ';
	}

	public mapPreviewDetails(): string {
		return 'Մանրամասն';
	}

	public rowSizeLabel(): string {
		return 'Ծառերի քանակը';
	}

	public rowStepInfo(count: number, step: number): string {
		return `Դուք կտնկեք ${count} ծառ՝ ${step.toFixed(1)} մետր միջակայքով։`;
	}

	public addRowConfirmButton(): string {
		return 'Ավելացնել';
	}

	public rowLength(value: number): string {
		return `Երկարություն: ${value.toFixed(1)} ն`;
	}

	public addContinueButton(): string {
		return 'Շարունակել';
	}

	public addRowIntro(): string {
		return 'Տեղաշարժե՛ք քարտեզը՝ շարքի վերջը նշելու համար։';
	}

	public globalHeatmapHeader(): string {
		return 'Վերջին ակտիվություն';
	}

	public learnContributions(): string {
		return 'Իմացեք, թե ինչպես ենք մենք հաշվում ներդրումները';
	}

	public uploadsTitle(): string {
		return 'Վերբեռնումներ';
	}

	public uploadsDate(): string {
		return 'Ամսաթիվ';
	}

	public uploadsStatus(): string {
		return 'Կարգավիճակ';
	}

	public uploadsSize(): string {
		return 'Չափ';
	}

	public uploadsAuto(): string {
		return 'Ֆայլերը վերբեռնել ավտոմատ';
	}

	public toastErrorAddingComment(): string {
		return 'Մեկնաբանություն ավելացնելու սխալ:';
	}
}

type LangCode = 'en' | 'ru' | 'hy';

export const getLanguageCode = (): LangCode => {
	const languages = navigator.languages || [navigator.language || 'en-US'];

	for (const lang of languages) {
		if (lang.startsWith('ru')) {
			return 'ru';
		}

		if (lang.startsWith('hy')) {
			return 'hy';
		}
	}

	return 'en';
};

const guessLocale = () => {
	const lang = getLanguageCode();

	if (lang == 'ru') {
		return new RussianLocale();
	}

	if (lang == 'hy') {
		return new ArmenianLocale();
	}

	return new EnglishLocale();
};

type LangList<T> = {
	[key in LangCode]: T;
};

export const localize = <T>(args: LangList<T>): T => {
	const lang = getLanguageCode();
	return args[lang];
};

export const locale = guessLocale();
