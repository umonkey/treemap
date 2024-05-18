import axios, { AxiosInstance, AxiosRequestConfig } from "axios";

import {
  IAddTreesRequest,
  IApiError,
  IComment,
  IFileStatusResponse,
  IFileUploadResponse,
  ILatLng,
  ISpecies,
  ITreeDefaults,
  ITreeDetails,
  ITreeInfo,
  ITreeStats,
  IUpdateTreeRequest,
  IUserInfo,
} from "@/types";

import { getApiRoot } from "@/utils/env";

export interface ITreesResponse {
  trees: ITreeInfo[];
}

export class TreeMapService {
  private readonly root: string;

  private readonly client: AxiosInstance;

  private token: string | null = null;

  public constructor() {
    this.root = getApiRoot();

    this.client = axios.create({
      baseURL: this.root,
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
    search?: string;
  }): Promise<ITreeInfo[]> {
    const res = await this.get<ITreesResponse>("/v1/trees", {
      params: {
        n: params.north,
        e: params.east,
        s: params.south,
        w: params.west,
        search: params.search || "",
      }
    });

    return res.trees;
  }

  /**
   * Add a new tree to the map.
   */
  public async addTrees(props: IAddTreesRequest): Promise<ITreeInfo[]> {
    const res = await this.post<ITreesResponse>("/v1/trees", props, {
      headers: this.get_auth_headers(),
    });

    return res.trees;
  }

  /**
   * Update an existing tree.
   */
  public async updateTree(id: string, props: IUpdateTreeRequest): Promise<ITreeInfo> {
    const res = await this.put<ITreeInfo>(`/v1/trees/${id}`, props, {
      headers: this.get_auth_headers(),
    });

    return res;
  }

  public async getTreeDetails(id: string): Promise<ITreeDetails> {
    return await this.get<ITreeDetails>(`/v1/trees/${id}`);
  }

  public async getTreeStats(): Promise<ITreeStats> {
    return await this.get<ITreeStats>("/v1/trees/stats");
  }

  public async updateTreePosition(id: string, position: ILatLng) {
    await this.put(`/v1/trees/${id}/position`, {
      lat: position.lat,
      lon: position.lon,
    }, {
      headers: this.get_auth_headers(),
    });
  }

  public async getTreeDefaults(): Promise<ITreeDefaults> {
    return await this.get(`/v1/trees/defaults`, {
      headers: this.get_auth_headers(),
    });
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

    this.setToken(res.token);

    return res;
  }

  public async getUserInfo(): Promise<IUserInfo> {
    return await this.get<IUserInfo>("/v1/me", {
      headers: this.get_auth_headers(),
    });
  }

  public async uploadImage({
    tree_id,
    file,
    progress,
  }: {
    tree_id: string,
    file: File,
    progress: (total: number, sent: number) => void,
  }): Promise<IFileUploadResponse> {
    const buffer = await file.arrayBuffer();
    const body = new Blob([buffer], { type: file.type });

    return await this.post(`/v1/trees/${tree_id}/files`, body, {
      headers: this.get_auth_headers(),
      timeout: 60000,

      onUploadProgress: (event) => {
        if (event.total && event.bytes) {
          progress(event.bytes, event.total);
        }
      },
    });
  }

  public async getFileStatus(file_id: string): Promise<IFileStatusResponse> {
    return await this.get(`/v1/files/${file_id}/status`);
  }

  public async addComment(tree_id: string, text: string): Promise<IComment[]> {
    return await this.post(`/v1/trees/${tree_id}/comments`, {
      message: text,
    }, {
      headers: this.get_auth_headers(),
    });
  }

  public async getComments(tree_id: string): Promise<IComment[]> {
    return await this.get<IComment[]>(`/v1/trees/${tree_id}/comments`);
  }

  public async searchSpecies(query: string): Promise<ISpecies[]> {
    const params = new URLSearchParams();
    params.append("query", query);
    return await this.get<ISpecies[]>(`/v1/species/search?${params.toString()}`);
  }

  public async suggestSpecies(): Promise<string[]> {
    return await this.get<string[]>("/v1/species/suggest", {
      headers: this.get_auth_headers(),
    });
  }

  public setToken(value: string | null): void {
    this.token = value;
    console.debug("[api] Token updated.");
  }

  private async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    try {
      const res = await this.client.get<T>(url, config);
      return res.data;
    } catch (e) {
      throw this.convert_error(e);
    }
  }

  private async post<T>(url: string, data: object, config?: AxiosRequestConfig): Promise<T> {
    try {
      const res = await this.client.post<T>(url, data, config);
      return res.data;
    } catch (e) {
      throw this.convert_error(e);
    }
  }

  private async put<T>(url: string, data: object, config?: AxiosRequestConfig): Promise<T> {
    try {
      const res = await this.client.put<T>(url, data, config);
      return res.data;
    } catch (e) {
      throw this.convert_error(e);
    }
  }

  private convert_error(e: unknown): IApiError {
    // @ts-expect-error TS18046
    const status = e.response?.status ?? 500;
    // @ts-expect-error TS18046
    const code = e.response?.data?.error?.code ?? "UnknownError";
    // @ts-expect-error TS18046
    const message = e.response?.data?.error?.description ?? e.message ?? "Something went wrong, please try again later.";

    return { status, code, message };
  }

  private get_auth_headers(): Record<string, string> {
    if (this.token !== null) {
      return {
        "Authorization": `Bearer ${this.token}`,
      };
    }

    return {};
  }
}
