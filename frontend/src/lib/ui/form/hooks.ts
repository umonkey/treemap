type Props = {
	onSubmit?: () => void;
};

export const hooks = ({ onSubmit }: Props) => {
	const handleSubmit = () => {
		if (onSubmit) {
			onSubmit();
		}
	};

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.key === 'Enter' && event.ctrlKey) {
			if (onSubmit) {
				onSubmit();
			}
		}
	};

	return { handleSubmit, handleKeyDown };
};
