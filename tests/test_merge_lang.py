import contextlib
import io
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from tools import merge_lang


class TranslationMergeTests(unittest.TestCase):
    def test_independent_updates_additions_and_deletions_are_preserved(self):
        result, conflicts = merge_lang.merge(
            {"upstream": "old", "fork": "old", "delete": "old", "same": "old"},
            {"upstream": "old", "fork": "local", "delete": "old", "same": "new",
             "offline": "Hors ligne"},
            {"upstream": "translated", "fork": "old", "same": "new", "new": "incoming"},
        )
        self.assertEqual(conflicts, [])
        self.assertEqual(result, {"upstream": "translated", "fork": "local", "same": "new",
                                  "offline": "Hors ligne", "new": "incoming"})

    def test_same_key_edits_and_delete_edit_conflicts_require_resolution(self):
        for base, ours, theirs in [
            ({"key": "old"}, {"key": "local"}, {"key": "incoming"}),
            ({"key": "old"}, {}, {"key": "incoming"}),
            ({"key": "old"}, {"key": "local"}, {}),
            ({}, {"key": "local"}, {"key": "incoming"}),
        ]:
            with self.subTest(base=base, ours=ours, theirs=theirs):
                self.assertEqual(merge_lang.merge(base, ours, theirs)[1], ["key"])

    def test_invalid_or_conflicted_driver_input_does_not_overwrite_ours(self):
        with tempfile.TemporaryDirectory() as tmp:
            paths = [Path(tmp) / name for name in ("base", "ours", "theirs")]
            for p, value in zip(paths, ("old", "local", "incoming")):
                p.write_text(json.dumps({"key": value}), encoding="utf-8")
            before = paths[1].read_bytes()
            with contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(merge_lang.main(paths), 1)
                for invalid in ('{"key":"a","key":"b"}', '{"key":{}}', '[]', 'invalid'):
                    paths[2].write_text(invalid, encoding="utf-8")
                    self.assertEqual(merge_lang.main(paths), 2)
            self.assertEqual(paths[1].read_bytes(), before)

    def test_git_uses_the_driver_to_merge_reordered_translation_tables(self):
        script = Path(merge_lang.__file__).resolve().as_posix()
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)

            def git(*args):
                return subprocess.run(["git", *args], cwd=root, check=True,
                                      capture_output=True, text=True).stdout

            def commit(values):
                (root / "lang.json").write_text(json.dumps(values, ensure_ascii=False) + "\n",
                                                encoding="utf-8")
                git("add", "lang.json")
                git("commit", "-m", "fixture")

            git("init", "--initial-branch=main")
            git("config", "user.name", "Merge fixture")
            git("config", "user.email", "fixture@example.invalid")
            driver = f'"{Path(sys.executable).as_posix()}" "{script}" %O %A %B'
            git("config", "merge.horizon-lang.driver", driver)
            (root / ".gitattributes").write_text("lang.json merge=horizon-lang\n")
            git("add", ".gitattributes")
            commit({"label": "old", "stable": "stable"})
            git("branch", "upstream")
            commit({"offline": "Hors ligne", "stable": "stable", "label": "old"})
            git("checkout", "upstream")
            commit({"label": "Étiquette", "stable": "stable"})
            git("checkout", "main")
            git("merge", "--no-edit", "upstream")
            self.assertEqual(json.loads((root / "lang.json").read_text(encoding="utf-8")),
                             {"label": "Étiquette", "stable": "stable", "offline": "Hors ligne"})
            self.assertEqual(git("status", "--porcelain"), "")

    def test_explicit_local_resolution_retains_independent_upstream_changes(self):
        with tempfile.TemporaryDirectory() as tmp:
            paths = [Path(tmp) / name for name in ("base", "ours", "theirs")]
            tables = ({"deleted": "old", "label": "old"}, {"label": "old"},
                      {"deleted": "translated", "label": "translated"})
            for path, values in zip(paths, tables):
                path.write_text(json.dumps(values), encoding="utf-8")
            self.assertEqual(merge_lang.main(["--keep-local-conflicts", *paths]), 0)
            self.assertEqual(json.loads(paths[1].read_text(encoding="utf-8")),
                             {"label": "translated"})
