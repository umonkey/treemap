import axios, { AxiosResponse, AxiosInstance, AxiosRequestConfig } from "axios";

import { ITreeInfo } from "./types";

export class TreeMapService {
  private readonly client: AxiosInstance;

  public constructor() {
    this.client = axios.create({
      baseURL: "http://localhost:8000",
      timeout: 10000,
      responseType: "json",
      headers: {
        "Content-Type": "application/json",
      },
    });
  }

  public async getMarkers(): Promise<ITreeInfo[]> {
    const res = await this.get("/v1/trees");
    return res.data.trees;
  }

  private async get<T>(url: string, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
    return this.client.get<T>(url, config);
  }
}
