import axios, { AxiosInstance, AxiosRequestConfig } from "axios";

import { IAddTreeRequest, ITreeInfo, ITreeDetails, IUserInfo } from "@/types";
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
  public async addMarker(props: IAddTreeRequest, token: string): Promise<ITreeInfo> {
    const res = await this.post<ITreeInfo>("/v1/trees", props, {
      headers: {
        "Authorization": `Bearer ${token}`,
      },
    });

    return res;
  }

  /**
   * Update an existing tree.
   */
  public async updateTree(props: ITreeDetails, token: string): Promise<ITreeInfo> {
    const res = await this.put<ITreeInfo>(`/v1/trees/${props.id}`, props, {
      headers: {
        "Authorization": `Bearer ${token}`,
      },
    });

    return res;
  }

  public async getTreeDetails(id: string): Promise<ITreeDetails> {
    return await this.get<ITreeDetails>(`/v1/trees/${id}`);
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

  private async put<T>(url: string, data: object, config?: AxiosRequestConfig): Promise<T> {
    const res = await this.client.put<T>(url, data, config);
    return res.data;
  }
}
