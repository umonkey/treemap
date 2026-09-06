"""
Aligns OpenSfM reconstruction trajectory with GPS track (.gpx) and
outputs trajectory.geojson.
"""

import json
import math
import os
import re
from datetime import datetime, timezone
from typing import Any, Dict, List, Optional, Tuple

import numpy as np

from .speed import parse_gpx


def lla_to_enu(
    lat: float,
    lon: float,
    alt: float,
    lat_ref: float,
    lon_ref: float,
    alt_ref: float,
) -> Tuple[float, float, float]:
    """Convert WGS84 (lat, lon, alt) to local ENU coordinates in meters."""
    a = 6378137.0
    f = 1.0 / 298.257223563
    b = (1.0 - f) * a
    e2 = (a**2 - b**2) / a**2

    lat_ref_rad = math.radians(lat_ref)

    sin_lat_ref = math.sin(lat_ref_rad)
    cos_lat_ref = math.cos(lat_ref_rad)
    n_rad = a / math.sqrt(1.0 - e2 * sin_lat_ref**2)

    dlat = math.radians(lat - lat_ref)
    dlon = math.radians(lon - lon_ref)

    n = dlat * (n_rad * (1.0 - e2) / (1.0 - e2 * sin_lat_ref**2))
    e = dlon * (n_rad * cos_lat_ref)
    u = alt - alt_ref
    return e, n, u


def enu_to_lla(
    e: float,
    n: float,
    u: float,
    lat_ref: float,
    lon_ref: float,
    alt_ref: float,
) -> Tuple[float, float, float]:
    """Convert local ENU coordinates in meters to WGS84 (lat, lon, alt)."""
    a = 6378137.0
    f = 1.0 / 298.257223563
    b = (1.0 - f) * a
    e2 = (a**2 - b**2) / a**2

    lat_ref_rad = math.radians(lat_ref)

    sin_lat = math.sin(lat_ref_rad)
    cos_lat = math.cos(lat_ref_rad)
    n_rad = a / math.sqrt(1.0 - e2 * sin_lat**2)

    dlat = n / (n_rad * (1.0 - e2) / (1.0 - e2 * sin_lat**2))
    dlon = e / (n_rad * cos_lat)

    lat = lat_ref + math.degrees(dlat)
    lon = lon_ref + math.degrees(dlon)
    alt = alt_ref + u
    return lat, lon, alt


def rodrigues(r_vec: np.ndarray) -> np.ndarray:
    """
    Convert a 3D rotation vector (axis-angle) to a 3x3 rotation matrix
    using Rodrigues' formula in pure NumPy.
    """
    theta = float(np.linalg.norm(r_vec))
    if theta < 1e-12:
        return np.eye(3, dtype=np.float64)

    u = r_vec / theta
    k_mat = np.array(
        [[0.0, -u[2], u[1]], [u[2], 0.0, -u[0]], [-u[1], u[0], 0.0]], dtype=np.float64
    )
    return (
        np.eye(3, dtype=np.float64)
        + math.sin(theta) * k_mat
        + (1.0 - math.cos(theta)) * (k_mat @ k_mat)
    )


def umeyama_2d(
    p: np.ndarray, q: np.ndarray, weights: Optional[np.ndarray] = None
) -> Tuple[float, float, np.ndarray, np.ndarray]:
    """
    Weighted 2D Umeyama similarity transformation from source p to target q.
    Minimizes sum_i w_i || q_i - (s * R * p_i + t) ||^2.

    Returns:
        scale (s): float
        theta (rotation angle in radians): float
        R (2x2 rotation matrix): np.ndarray
        t (2D translation vector): np.ndarray
    """
    n = len(p)
    if weights is None:
        w = np.ones(n, dtype=np.float64) / n
    else:
        total_w = float(np.sum(weights))
        if total_w < 1e-12:
            w = np.ones(n, dtype=np.float64) / n
        else:
            w = weights.astype(np.float64) / total_w

    p_mean = np.sum(p * w[:, None], axis=0)
    q_mean = np.sum(q * w[:, None], axis=0)

    p_c = p - p_mean
    q_c = q - q_mean

    # Weighted covariance matrix S = (q_c * w[:, None]).T @ p_c
    s_mat = (q_c * w[:, None]).T @ p_c
    s_ex, s_ez = float(s_mat[0, 0]), float(s_mat[0, 1])
    s_nx, s_nz = float(s_mat[1, 0]), float(s_mat[1, 1])

    a = s_ex + s_nz
    b = s_nx - s_ez
    theta = math.atan2(b, a)

    cos_th = math.cos(theta)
    sin_th = math.sin(theta)
    r_mat = np.array([[cos_th, -sin_th], [sin_th, cos_th]], dtype=np.float64)

    var_p = float(np.sum(w[:, None] * (p_c**2)))
    if var_p < 1e-12:
        scale = 1.0
    else:
        scale = (a * cos_th + b * sin_th) / var_p

    t = q_mean - scale * (r_mat @ p_mean)
    return float(scale), float(theta), r_mat, t


def fit_irls(
    p: np.ndarray,
    q: np.ndarray,
    c: float = 3.0,
    max_iter: int = 15,
    tol: float = 1e-4,
) -> Tuple[float, float, np.ndarray, np.ndarray, np.ndarray, float, np.ndarray]:
    """
    Iteratively Reweighted Least Squares (IRLS) with Cauchy loss:
    w_i = 1 / (1 + (residual / c)^2).
    """
    weights = np.ones(len(p), dtype=np.float64)
    scale, theta, r_2d, t_xy = umeyama_2d(p, q, weights)

    for _ in range(max_iter):
        pred = scale * (p @ r_2d.T) + t_xy
        res = np.linalg.norm(q - pred, axis=1)
        new_weights = 1.0 / (1.0 + (res / c) ** 2)
        if float(np.max(np.abs(new_weights - weights))) < tol:
            weights = new_weights
            break
        weights = new_weights
        scale, theta, r_2d, t_xy = umeyama_2d(p, q, weights)

    pred = scale * (p @ r_2d.T) + t_xy
    res = np.linalg.norm(q - pred, axis=1)
    loss = float(np.sum((c**2 / 2.0) * np.log(1.0 + (res / c) ** 2)))
    return scale, theta, r_2d, t_xy, res, loss, weights


def get_heading_pitch_roll(r_c2w: np.ndarray) -> Tuple[float, float, float]:
    """
    Calculate geographic heading, pitch, and roll in degrees from camera-to-world
    rotation matrix R'_c2w (transforming camera coordinates [right, down, forward]
    to True North ENU [East, North, Up]).

    Returns:
        heading: [0, 360) degrees clockwise from North
        pitch: [-90, 90] degrees (up is positive)
        roll: [-180, 180] degrees (right side down is positive)
    """
    v_forward = r_c2w @ np.array([0.0, 0.0, 1.0], dtype=np.float64)
    v_right = r_c2w @ np.array([1.0, 0.0, 0.0], dtype=np.float64)

    heading = math.degrees(math.atan2(v_forward[0], v_forward[1])) % 360.0
    pitch = math.degrees(math.asin(float(np.clip(v_forward[2], -1.0, 1.0))))

    world_up = np.array([0.0, 0.0, 1.0], dtype=np.float64)
    h_right = np.cross(v_forward, world_up)
    h_right_len = float(np.linalg.norm(h_right))

    if h_right_len < 1e-6:
        roll = 0.0
    else:
        u_h_right = h_right / h_right_len
        u_h_up = np.cross(u_h_right, v_forward)
        sin_roll = -float(np.dot(v_right, u_h_up))
        cos_roll = float(np.dot(v_right, u_h_right))
        roll = math.degrees(math.atan2(sin_roll, cos_roll))

    return heading, pitch, roll


def format_utc_time(dt: datetime) -> str:
    """Format UTC datetime to ISO 8601 string ending with Z."""
    if dt.microsecond != 0:
        return dt.strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"
    return dt.strftime("%Y-%m-%dT%H:%M:%SZ")


def run_align_trajectory(dataset_path: str) -> None:
    """
    Align SfM reconstruction trajectory with GPS track (.gpx) and
    write trajectory.geojson. Strictly non-destructive: does not modify
    reconstruction.json or track.gpx, and does not write reference_lla.json.
    """
    reconstruction_path = os.path.join(dataset_path, "reconstruction.json")
    if not os.path.exists(reconstruction_path):
        print(
            f"Warning: reconstruction.json not found at {reconstruction_path}, "
            "skipping alignment."
        )
        return

    gpx_path = os.path.join(dataset_path, "track.gpx")
    if not os.path.exists(gpx_path):
        print(f"Warning: track.gpx not found at {gpx_path}, skipping alignment.")
        return

    print(f"Loading GPX track from {gpx_path}...")
    gpx_points = parse_gpx(gpx_path)
    if not gpx_points:
        print("Warning: No timestamped points found in track.gpx, skipping alignment.")
        return

    print(f"Loaded {len(gpx_points)} GPS points from {gpx_path}.")

    t_gpx_start = gpx_points[0][0]
    lat_ref, lon_ref, alt_ref = (
        gpx_points[0][1],
        gpx_points[0][2],
        gpx_points[0][3],
    )
    gpx_times = np.array([p[0] - t_gpx_start for p in gpx_points], dtype=np.float64)
    gpx_enu = np.array(
        [lla_to_enu(p[1], p[2], p[3], lat_ref, lon_ref, alt_ref) for p in gpx_points],
        dtype=np.float64,
    )

    def interp_gpx_enu(t_eval: np.ndarray) -> np.ndarray:
        e = np.interp(t_eval, gpx_times, gpx_enu[:, 0])
        n = np.interp(t_eval, gpx_times, gpx_enu[:, 1])
        u = np.interp(t_eval, gpx_times, gpx_enu[:, 2])
        return np.column_stack([e, n, u])

    print(f"Reading SfM reconstruction from {reconstruction_path} (read-only)...")
    with open(reconstruction_path, "r") as f:
        reconstructions: List[Dict[str, Any]] = json.load(f)

    if not reconstructions:
        print("Warning: reconstruction.json is empty, skipping alignment.")
        return

    # Select largest reconstruction component
    recon = max(reconstructions, key=lambda r: len(r.get("shots", {})))
    shots = recon.get("shots", {})
    if not shots:
        print("Warning: No shots found in reconstruction, skipping alignment.")
        return

    video_duration: Optional[float] = None
    video_path = os.path.join(dataset_path, "video.mp4")
    if os.path.exists(video_path):
        try:
            import av  # type: ignore

            container = av.open(video_path)
            stream = container.streams.video[0]
            if stream.duration and stream.time_base:
                video_duration = float(stream.duration * stream.time_base)
            container.close()
        except Exception:
            pass

    # Extract shot indices, camera centers, and capture times
    shot_items: List[Tuple[int, str, np.ndarray, np.ndarray, float]] = []
    for shot_id, shot_data in shots.items():
        m = re.findall(r"\d+", shot_id)
        index = int(m[-1]) if m else 1

        r_vec = np.array(shot_data["rotation"], dtype=np.float64)
        t_vec = np.array(shot_data["translation"], dtype=np.float64)

        r_cam = rodrigues(r_vec)
        c_sfm = -r_cam.T @ t_vec
        capture_time = float(shot_data.get("capture_time", 0.0))
        shot_items.append((index, shot_id, c_sfm, r_cam, capture_time))

    # Sort shots by frame index
    shot_items.sort(key=lambda s: s[0])
    total_shots = len(shot_items)

    indices = [s[0] for s in shot_items]
    shot_ids = [s[1] for s in shot_items]
    c_sfm_arr = np.array([s[2] for s in shot_items], dtype=np.float64)
    r_cam_arr = [s[3] for s in shot_items]
    capture_times = np.array([s[4] for s in shot_items], dtype=np.float64)

    # Calculate frame timestamps relative to video start:
    if len(capture_times) > 0 and np.any(capture_times > 0):
        sfm_t = capture_times - capture_times[0]
    else:
        # Fallback if capture_time is missing: assume 30 fps and frame_interval 10
        sfm_t = np.array([(idx - 1) * 10 / 30.0 for idx in indices], dtype=np.float64)

    if video_duration is None:
        video_duration = float(sfm_t[-1] - sfm_t[0]) if len(sfm_t) > 1 else 0.0

    p_2d = c_sfm_arr[:, [0, 2]]  # OpenSfM horizontal plane: X and Z

    gpx_duration = float(gpx_times[-1] - gpx_times[0])
    video_span = float(sfm_t[-1] - sfm_t[0])
    max_tau = max(0.0, gpx_duration - video_span)

    print(
        f"Alignment window: video span {video_span:.1f}s across GPX "
        f"{gpx_duration:.1f}s (max start offset tau: {max_tau:.1f}s)."
    )

    # 1. Coarse sliding window search for tau_0
    best_rmse = float("inf")
    best_tau_0 = 0.0

    tau_step = 0.5
    num_steps = max(1, int(math.ceil(max_tau / tau_step)))
    tau_candidates = np.linspace(0.0, max_tau, num_steps + 1)

    for tau_cand in tau_candidates:
        q_cand = interp_gpx_enu(sfm_t + tau_cand)[:, :2]
        s_c, th_c, r_c, t_c = umeyama_2d(p_2d, q_cand)
        pred_q = s_c * (p_2d @ r_c.T) + t_c
        rmse = float(np.sqrt(np.mean(np.sum((q_cand - pred_q) ** 2, axis=1))))
        if rmse < best_rmse:
            best_rmse = rmse
            best_tau_0 = float(tau_cand)

    print(
        f"Coarse search completed: tau_0 = {best_tau_0:.2f}s, "
        f"RMSE = {best_rmse:.2f}m"
    )

    # 2. Refine tau_0 and (scale, theta, tx, ty) using IRLS with Cauchy loss
    search_min = max(0.0, best_tau_0 - 4.0)
    search_max = min(max_tau, best_tau_0 + 4.0)
    fine_tau_candidates = np.arange(search_min, search_max + 1e-6, 0.1)

    best_loss = float("inf")
    refined_tau = best_tau_0

    for tau_cand in fine_tau_candidates:
        q_cand = interp_gpx_enu(sfm_t + tau_cand)[:, :2]
        _, _, _, _, _, loss, _ = fit_irls(p_2d, q_cand, c=3.0)
        if loss < best_loss:
            best_loss = loss
            refined_tau = float(tau_cand)

    # Final IRLS fit with refined tau
    q_target_enu = interp_gpx_enu(sfm_t + refined_tau)
    scale, theta, r_2d, t_xy, res_2d, loss, weights = fit_irls(
        p_2d, q_target_enu[:, :2], c=3.0
    )

    # 3. Match vertical altitude offset tu using median GPX elevation over active window
    med_u_gpx = float(np.median(q_target_enu[:, 2]))
    # U = s * (-Y) + tu => tu = med_u_gpx + scale * median(Y)
    tu = med_u_gpx + scale * float(np.median(c_sfm_arr[:, 1]))

    # Rigid world transformation R_world with horizontal yaw theta and vertical U = -Y
    cos_th = math.cos(theta)
    sin_th = math.sin(theta)
    r_world = np.array(
        [[cos_th, 0.0, -sin_th], [sin_th, 0.0, cos_th], [0.0, -1.0, 0.0]],
        dtype=np.float64,
    )
    t_3d = np.array([t_xy[0], t_xy[1], tu], dtype=np.float64)

    # Camera centers in ENU: C'_enu = s * R_world * C_sfm + T
    c_enu_all = scale * (c_sfm_arr @ r_world.T) + t_3d

    # Speeds between consecutive camera positions
    speeds: List[float] = []
    for i in range(total_shots):
        if i > 0:
            dt = sfm_t[i] - sfm_t[i - 1]
            dist = float(np.linalg.norm(c_enu_all[i] - c_enu_all[i - 1]))
            speeds.append(dist / dt if dt > 0 else 0.0)
        elif total_shots > 1:
            dt = sfm_t[1] - sfm_t[0]
            dist = float(np.linalg.norm(c_enu_all[1] - c_enu_all[0]))
            speeds.append(dist / dt if dt > 0 else 0.0)
        else:
            speeds.append(0.0)

    # Total distance and average speed
    total_distance_m = 0.0
    for i in range(total_shots - 1):
        total_distance_m += float(np.linalg.norm(c_enu_all[i + 1] - c_enu_all[i]))

    avg_speed_mps = total_distance_m / video_span if video_span > 0 else 0.0

    # Convert ENU to LLA and build Point features
    point_features: List[Dict[str, Any]] = []
    line_coords: List[List[float]] = []

    for i in range(total_shots):
        e_i, n_i, u_i = c_enu_all[i]
        lat_i, lon_i, alt_i = enu_to_lla(e_i, n_i, u_i, lat_ref, lon_ref, alt_ref)
        line_coords.append([round(lon_i, 7), round(lat_i, 7), round(alt_i, 2)])

        # Orientation: R'_c2w = R_world * R_cam^T
        r_c2w = r_world @ r_cam_arr[i].T
        heading, pitch, roll = get_heading_pitch_roll(r_c2w)

        shot_utc_dt = datetime.fromtimestamp(
            t_gpx_start + refined_tau + sfm_t[i], tz=timezone.utc
        )

        pt_feat = {
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [
                    round(lon_i, 7),
                    round(lat_i, 7),
                    round(alt_i, 2),
                ],
            },
            "properties": {
                "shot_id": shot_ids[i],
                "index": indices[i],
                "time_offset_sec": round(float(sfm_t[i]), 3),
                "utc_time": format_utc_time(shot_utc_dt),
                "heading": round(heading, 1),
                "pitch": round(pitch, 1),
                "roll": round(roll, 1),
                "altitude_m": round(alt_i, 2),
                "speed_mps": round(speeds[i], 2),
                "gps_residual_m": round(float(res_2d[i]), 2),
            },
        }
        point_features.append(pt_feat)

    # LineString feature connecting all points
    linestring_feature = {
        "type": "Feature",
        "geometry": {
            "type": "LineString",
            "coordinates": line_coords,
        },
        "properties": {
            "name": "Camera Trajectory",
            "total_distance_m": round(total_distance_m, 2),
            "avg_speed_mps": round(avg_speed_mps, 2),
        },
    }

    gpx_start_dt = datetime.fromtimestamp(t_gpx_start, tz=timezone.utc)
    video_start_dt = datetime.fromtimestamp(t_gpx_start + refined_tau, tz=timezone.utc)

    median_residual = float(np.median(res_2d))

    geojson_data: Dict[str, Any] = {
        "type": "FeatureCollection",
        "metadata": {
            "video_duration_sec": round(video_duration, 3),
            "gpx_start_time": format_utc_time(gpx_start_dt),
            "gpx_offset_sec": round(refined_tau, 3),
            "video_start_utc": format_utc_time(video_start_dt),
            "scale_meters_per_unit": round(scale, 6),
            "rotation_yaw_deg": round(math.degrees(theta), 2),
            "translation_enu": [
                round(float(t_xy[0]), 3),
                round(float(t_xy[1]), 3),
                round(float(tu), 3),
            ],
            "alignment_median_residual_m": round(median_residual, 3),
            "total_shots": total_shots,
        },
        "features": [linestring_feature] + point_features,
    }

    output_geojson_path = os.path.join(dataset_path, "trajectory.geojson")
    with open(output_geojson_path, "w") as f:
        json.dump(geojson_data, f, indent=2)

    print(f"Alignment completed: saved trajectory to {output_geojson_path}")
    print(
        f"Metrics: shots={total_shots}, scale={scale:.4f}, "
        f"yaw={math.degrees(theta):.2f}°, tau={refined_tau:.2f}s, "
        f"median_res={median_residual:.2f}m, dist={total_distance_m:.1f}m"
    )
