import axios, { AxiosError, AxiosInstance, AxiosRequestConfig } from "axios";

import { IApiError, IAddTreeRequest, ILatLng, ITreeInfo, ITreeDetails, IUploadTicket, IUserInfo } from "@/types";
import { getUserToken } from "@/utils/userinfo";
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
  public async addMarker(props: IAddTreeRequest): Promise<ITreeInfo> {
    const res = await this.post<ITreeInfo>("/v1/trees", props, {
      headers: this.get_auth_headers(),
    });

    return res;
  }

  /**
   * Update an existing tree.
   */
  public async updateTree(props: ITreeDetails): Promise<ITreeInfo> {
    const res = await this.put<ITreeInfo>(`/v1/trees/${props.id}`, props, {
      headers: this.get_auth_headers(),
    });

    return res;
  }

  public async getTreeDetails(id: string): Promise<ITreeDetails> {
    return await this.get<ITreeDetails>(`/v1/trees/${id}`);
  }

  public async updateTreePosition(id: string, position: ILatLng) {
    await this.put(`/v1/trees/${id}/position`, {
      lat: position.lat,
      lon: position.lon,
    }, {
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

    return res;
  }

  /**
   * Request an upload ticket for uploading images.
   */
  public async createUploadTicket(): Promise<IUploadTicket> {
    return await this.post<IUploadTicket>("/v1/uploads", { }, {
      headers: this.get_auth_headers(),
    });
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
    if (e instanceof AxiosError) {
      return {
        status: e.response?.status ?? 500,
        code: e.response?.data?.error?.code ?? "UnknownError",
        message: e.response?.data?.error?.description ?? "Something went wrong, please try again later.",
      } as IApiError;
    }

    if (e instanceof Error) {
      return {
        status: 500,
        code: "UnknownError",
        message: e.message,
      } as IApiError;
    }

    return {
      status: 500,
      code: "UnknownError",
      message: "Something went wrong, please try again later.",
    } as IApiError;
  }

  private get_auth_headers(): Record<string, string> {
      const token = getUserToken();

      if (token !== null) {
        return {
          "Authorization": `Bearer ${token}`,
        };
      }

      return {};
  }
}
