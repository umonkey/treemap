/**
 * This is the error objec that we pass around the application.
 */
export interface IError {
	code: string;
	description: string;
}

export interface IResponse<T> {
	status: number;
	data?: T;
	error?: IError;
}

export class Response<T> {
	public status: number;
	public data?: T;
	public error?: IError;

	constructor(status: number, data?: T, error?: IError) {
		this.status = status;
		this.data = data;
		this.error = error;
	}

	public then(callback: (value: T) => void): Response<T> {
		if (this.status >= 200 && this.status < 400 && this.data) {
			callback(this.data);
		}

		return this;
	}

	public catch(callback: (error: IError) => void): Response<T> {
		if (this.status >= 400 && this.error) {
			callback(this.error);
		}

		return this;
	}

	public finally(callback: () => void): void {
		callback();
	}
}
