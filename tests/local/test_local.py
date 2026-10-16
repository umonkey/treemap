# Regression tests for the isolated TreeMap local development environment.
# Run with Python 3.9+ from the repository root using unittest discovery.
# Set TREEMAP_SMOKE=1 to also check running local Docker containers.
# Tests do not change the project's persistent SQLite database or containers.

import configparser
import json
import os
import re
import shutil
import sqlite3
import stat
import subprocess
import tempfile
import unittest
import urllib.error
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
COMPOSE = ("docker", "compose", "-f", "compose.yaml", "-f", "compose.local.yaml")
SMOKE = os.getenv("TREEMAP_SMOKE") == "1"


def command(*args, cwd=ROOT):
    """Run a command without a shell and return its stdout."""
    result = subprocess.run(
        args, cwd=cwd, text=True, capture_output=True, timeout=30, check=False
    )
    if result.returncode:
        raise AssertionError(
            f"Command failed: {' '.join(args)}\n{result.stderr.strip()}"
        )
    return result.stdout


def compose_config(*files):
    """Read the effective Compose configuration without starting containers."""
    result = command("docker", "compose", *files, "config", "--format", "json")
    return json.loads(result)


def supervisor_settings(path, section):
    parser = configparser.ConfigParser(interpolation=None)
    with path.open(encoding="utf-8") as source:
        parser.read_file(source)
    return parser[section]


def read_local_toml_settings(path):
    """Read the few flat TOML string settings checked by these tests.

    Python 3.9 has no tomllib. This targeted reader avoids requiring a
    third-party package; it is not intended to validate arbitrary TOML files.
    """
    pattern = re.compile(
        r"^\s*(database|file_storage|secrets_path|sqs_url)\s*=\s*"
        r"(?:\"([^\"]*)\"|'([^']*)')\s*(?:#.*)?$"
    )
    settings = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        match = pattern.match(line)
        if match:
            key, double_quoted, single_quoted = match.groups()
            settings[key] = double_quoted if double_quoted is not None else single_quoted
    return settings


class LocalFileTests(unittest.TestCase):
    """Offline checks. These never access the running development database."""

    def test_local_toml_disables_external_queue_and_storage(self):
        config_file = ROOT / "services/backend/config.toml.local"
        config = read_local_toml_settings(config_file)
        self.assertEqual(config["database"], "sqlite")
        self.assertEqual(config["file_storage"], "local")
        self.assertNotIn("sqs_url", config)
        self.assertEqual(config["secrets_path"], "/run/secrets")

    def test_cron_is_disabled_only_in_local_override(self):
        original = ROOT / "services/backend/docker/rootfs/etc/supervisor.d/crond.ini"
        local = ROOT / "services/backend/local/supervisor/crond.ini"
        self.assertTrue(supervisor_settings(original, "program:crond").getboolean("autostart"))
        options = supervisor_settings(local, "program:crond")
        self.assertFalse(options.getboolean("autostart"))
        self.assertFalse(options.getboolean("autorestart"))

    def test_local_email_logs_do_not_rotate_streams(self):
        path = ROOT / "services/backend/local/supervisor/dispatch-emails.ini"
        options = supervisor_settings(path, "program:dispatch-emails")
        self.assertEqual(options["stdout_logfile"], "/dev/stdout")
        self.assertEqual(options["stderr_logfile"], "/dev/stderr")
        self.assertEqual(options.getint("stdout_logfile_maxbytes"), 0)
        self.assertEqual(options.getint("stderr_logfile_maxbytes"), 0)

    def test_bootstrap_is_idempotent_and_preserves_trees(self):
        schema = (ROOT / "services/backend/dev/schema-sqlite.sql").read_text(encoding="utf-8")
        bootstrap = (ROOT / "services/backend/local/bootstrap.sql").read_text(encoding="utf-8")

        # Use an in-memory database; never modify the project's SQLite volume.
        with sqlite3.connect(":memory:") as database:
            database.executescript(schema)
            database.executescript(bootstrap)
            self.assertEqual(database.execute("SELECT COUNT(*) FROM trees").fetchone()[0], 0)
            self.assertEqual(
                database.execute("SELECT COUNT(*) FROM instances WHERE domain='localhost'").fetchone()[0],
                1,
            )
            permissions_before = database.execute(
                "SELECT COUNT(*) FROM role_permissions"
            ).fetchone()[0]
            self.assertGreater(permissions_before, 0)

            # Re-run the bootstrap after a contributor changes the local data.
            database.execute("UPDATE instances SET name='Custom local name' WHERE domain='localhost'")
            database.execute(
                "INSERT INTO trees (id, lat, lon, species, added_at, updated_at, updated_by, added_by) "
                "VALUES (1, 40.18, 44.51, 'Betula', 1, 1, 1, 1)"
            )
            database.commit()
            database.executescript(bootstrap)

            self.assertEqual(database.execute("SELECT COUNT(*) FROM trees").fetchone()[0], 1)
            self.assertEqual(
                database.execute("SELECT name FROM instances WHERE domain='localhost'").fetchone()[0],
                "Custom local name",
            )
            self.assertEqual(
                database.execute("SELECT COUNT(*) FROM role_permissions").fetchone()[0],
                permissions_before,
            )

    def test_local_startup_has_valid_shell_syntax(self):
        startup = ROOT / "services/backend/local/startup.sh"
        command("sh", "-n", str(startup))

    def test_secrets_initializer_is_repeatable(self):
        script = ROOT / "tools/init-local.sh"
        with tempfile.TemporaryDirectory(prefix="treemap-local-test-") as temp:
            temp_root = Path(temp)
            tools_dir = temp_root / "tools"
            tools_dir.mkdir()
            copied_script = tools_dir / "init-local.sh"
            shutil.copyfile(script, copied_script)

            # A temporary fake repository prevents changes to real secrets.
            command("sh", str(copied_script), cwd=temp_root)
            secrets = temp_root / ".local-secrets"
            jwt = secrets / "jwt_secret.txt"
            self.assertGreaterEqual(len(jwt.read_text(encoding="utf-8").strip()), 64)
            self.assertFalse(stat.S_IMODE(jwt.stat().st_mode) & 0o077)
            aws_key = secrets / "aws_key.txt"
            aws_key.write_text("do-not-overwrite\n", encoding="utf-8")
            original_jwt = jwt.read_bytes()

            command("sh", str(copied_script), cwd=temp_root)
            self.assertEqual(jwt.read_bytes(), original_jwt)
            self.assertEqual(aws_key.read_text(encoding="utf-8"), "do-not-overwrite\n")


@unittest.skipUnless(shutil.which("docker"), "Docker Compose executable not found")
class ComposeConfigTests(unittest.TestCase):
    """Validate the merged YAML without starting or stopping containers."""

    def test_dev_keeps_its_previous_configuration(self):
        dev = compose_config("-f", "compose.yaml")
        self.assertEqual(dev["name"], "trees_dev")
        self.assertEqual(
            dev["services"]["backend"]["environment"]["TREEMAP_CONFIG"],
            "/app/config.toml.dev",
        )

    def test_local_overrides_runtime_and_isolates_secrets(self):
        local = compose_config("-f", "compose.yaml", "-f", "compose.local.yaml")
        self.assertEqual(local["name"], "trees_local")
        backend = local["services"]["backend"]
        frontend = local["services"]["frontend"]
        self.assertEqual(backend["environment"]["TREEMAP_CONFIG"], "/app/config.toml.local")
        self.assertEqual(backend["command"], ["/bin/sh", "/app/startup-local.sh"])
        self.assertEqual(frontend["environment"]["PUBLIC_API_ROOT"], "http://localhost:8000/")

        backend_targets = {mount["target"] for mount in backend["volumes"]}
        frontend_targets = {mount["target"] for mount in frontend["volumes"]}
        self.assertIn("/etc/supervisor.d/crond.ini", backend_targets)
        self.assertIn("/app/bootstrap-local.sql", backend_targets)
        self.assertIn("/app/node_modules", frontend_targets)

        # Every referenced secret must come from .local-secrets, never .secrets.
        expected_directory = (ROOT / ".local-secrets").resolve()
        for key, definition in local["secrets"].items():
            self.assertIn("file", definition, f"Secret {key} must be file-backed")
            path = Path(definition["file"])
            if not path.is_absolute():
                path = ROOT / path
            self.assertEqual(path.resolve().parent, expected_directory, key)


@unittest.skipUnless(SMOKE, "Set TREEMAP_SMOKE=1 to test running local containers")
class LocalSmokeTests(unittest.TestCase):
    """Read-only checks against an already running local Compose project."""

    def test_running_services(self):
        running = command(*COMPOSE, "ps", "--services", "--status", "running").splitlines()
        self.assertIn("backend", running)
        self.assertIn("frontend", running)

    def test_backend_health(self):
        # Docker being up does not prove that the backend web process is listening.
        try:
            with urllib.request.urlopen("http://localhost:8000/health", timeout=6) as response:
                self.assertIn(response.status, (200, 204))
        except urllib.error.URLError as exc:
            self.fail(
                f"Backend API is unreachable at http://localhost:8000/health: {exc}. "
                "Check 'docker compose -f compose.yaml -f compose.local.yaml "
                "exec -T backend supervisorctl status web' and backend logs."
            )

    def test_frontend_http(self):
        with urllib.request.urlopen("http://localhost:5173/", timeout=6) as response:
            self.assertEqual(response.status, 200)
            self.assertIn("text/html", response.headers.get("Content-Type", ""))

    def test_local_instance_settings(self):
        request = urllib.request.Request(
            "http://localhost:8000/api/settings",
            headers={"Referer": "http://localhost:5173/"},
        )
        try:
            with urllib.request.urlopen(request, timeout=6) as response:
                self.assertEqual(response.status, 200)
                settings = json.load(response)
        except urllib.error.URLError as exc:
            self.fail(
                f"Local instance settings are unavailable: {exc}. "
                "Check the backend web process and its logs first."
            )
        self.assertIsInstance(settings["name"], str)
        self.assertTrue(settings["name"])
        self.assertEqual(len(settings["bbox"]), 4)

    def test_cron_stopped_and_local_config_loaded(self):
        # supervisorctl can return status 3 for STOPPED, which is expected here.
        result = subprocess.run(
            (*COMPOSE, "exec", "-T", "backend", "supervisorctl", "status", "crond"),
            cwd=ROOT, text=True, capture_output=True, timeout=30, check=False,
        )
        self.assertRegex(
            result.stdout, r"(?m)^crond\s+STOPPED\b",
            f"Expected local cron to be STOPPED. "
            f"exit={result.returncode}; stdout={result.stdout!r}; stderr={result.stderr!r}",
        )
        backend_config = command(*COMPOSE, "exec", "-T", "backend", "printenv", "TREEMAP_CONFIG")
        self.assertEqual(backend_config.strip(), "/app/config.toml.local")

    def test_backend_web_process(self):
        # The backend container may be RUNNING even when Supervisor's web task failed.
        result = subprocess.run(
            (*COMPOSE, "exec", "-T", "backend", "supervisorctl", "status", "web"),
            cwd=ROOT, text=True, capture_output=True, timeout=30, check=False,
        )
        self.assertRegex(
            result.stdout, r"(?m)^web\s+RUNNING\b",
            f"Backend web process is not RUNNING. "
            f"exit={result.returncode}; stdout={result.stdout!r}; stderr={result.stderr!r}. "
            "Inspect the backend logs for the failure reason.",
        )

    def test_local_database_contains_instance(self):
        # Force a machine-readable result, independent of the container's .sqliterc.
        count = command(
            *COMPOSE, "exec", "-T", "backend", "sqlite3", "-batch", "-noheader", "-list",
            "/app/var/database.sqlite",
            "SELECT COUNT(*) FROM instances WHERE domain='localhost';",
        )
        self.assertEqual(count.strip(), "1")
        # Do not require zero trees here: contributors may already have added data.

    def test_frontend_has_vite(self):
        command(*COMPOSE, "exec", "-T", "frontend", "sh", "-c", "test -x /app/node_modules/.bin/vite")


if __name__ == "__main__":
    unittest.main(verbosity=2)
