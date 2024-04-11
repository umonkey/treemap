import axios, { AxiosInstance, AxiosRequestConfig } from "axios";

import { ITreeInfo, ITreeDetails, IUserInfo } from "@/types";
import { getApiRoot } from "@/utils/env";

export interface ITreesResponse {
  trees: ITreeInfo[];
}

export class TreeMapService {
  private readonly client: AxiosInstance;

  public constructor() {
    this.client = axios.create({
      baseURL: getApiRoot(),
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

  /**
   * Add a new tree to the map.
   */
  public async addMarker(params: {
    lat: number;
    lon: number;
    species: string;
    height: number | null;
    circumference: number | null;
    diameter: number | null;
  }): Promise<ITreeInfo> {
    const res = await this.post<ITreeInfo>("/v1/trees", {
      lat: params.lat,
      lon: params.lon,
      name: params.species,
      height: params.height,
      circumference: params.circumference,
    });

    return res;
  }

  public async getTreeDetails(id: number): Promise<ITreeDetails> {
    return await this.get<ITreeDetails>(`/v1/tree/${id}`);
  }

  /**
   * Login with Google.
   *
   * Exchanges a Google OAuth token for a local user token.
   * Creates a new user account if necessary.
   */
  public async loginGoogle(token: string): Promise<IUserInfo> {
    const res = await this.post<IUserInfo>("/v1/login/google", {
      token,
    });

    return res;
  }

  private async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const res = await this.client.get<T>(url, config);
    return res.data;
  }

  private async post<T>(url: string, data: object, config?: AxiosRequestConfig): Promise<T> {
    const res = await this.client.post<T>(url, data, config);
    return res.data;
  }
}
