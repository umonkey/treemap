import axios, { AxiosInstance, AxiosRequestConfig } from "axios";

import { ITreeInfo } from "../../types";

interface ITreesResponse {
  trees: ITreeInfo[];
}

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

  public async getMarkers(params: {
    north: number;
    east: number;
    south: number;
    west: number;
  }): Promise<ITreeInfo[]> {
    const res = await this.get<ITreesResponse>("/v1/trees", {
      params: {
        n: params.north,
        e: params.east,
        s: params.south,
        w: params.west,
      }
    });

    return res.trees;
  }

  public async addMarker(params: {
    lat: number;
    lon: number;
    species: string;
  }): Promise<ITreeInfo> {
    const res = await this.post<ITreeInfo>("/v1/trees", {
      params: {
        lat: params.lat,
        lng: params.lon,
        name: params.species,
      },
    });

    return res;
  }

  private async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const res = await this.client.get<T>(url, config);
    return res.data;
  }

  private async post<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const res = await this.client.post<T>(url, config);
    return res.data;
  }
}
