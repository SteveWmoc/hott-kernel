import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import adversarial_review as review


def clean_report():
    return {
        "schema_version": "0.1",
        "verdict": "advisory_clear",
        "summary": "No evidence-backed defect was found.",
        "findings": [],
        "limitations": [],
    }


def finding_report(*, foundational=False):
    return {
        "schema_version": "0.1",
        "verdict": "foundational_stop" if foundational else "advisory_findings",
        "summary": "One issue was found.",
        "findings": [
            {
                "id": "AR-001",
                "severity": "P1",
                "title": "Boundary violation",
                "claim": "The implementation crosses a frozen boundary.",
                "requirement": "The format layer must not perform logical checking.",
                "evidence": [
                    {
                        "path": "src/format.rs",
                        "line": "12-16",
                        "detail": "A logical scope check is performed while decoding.",
                    }
                ],
                "reproduction": "Proposed, not executed: add an out-of-scope canonical fixture.",
                "confidence": "high",
                "foundational_change": foundational,
            }
        ],
        "limitations": ["No command was executed."],
    }


class ReportValidationTests(unittest.TestCase):
    def test_accepts_clean_report(self):
        self.assertEqual(review.validate_report(clean_report())["verdict"], "advisory_clear")

    def test_accepts_finding_report(self):
        report = finding_report()
        self.assertEqual(review.validate_report(report)["findings"][0]["id"], "AR-001")

    def test_requires_foundational_stop_for_foundational_finding(self):
        report = finding_report(foundational=True)
        report["verdict"] = "advisory_findings"
        with self.assertRaises(review.ReviewError):
            review.validate_report(report)

    def test_rejects_duplicate_finding_ids(self):
        report = finding_report()
        report["findings"].append(dict(report["findings"][0]))
        with self.assertRaises(review.ReviewError):
            review.validate_report(report)

    def test_rejects_unexpected_fields(self):
        report = clean_report()
        report["untrusted_extra"] = "ignored data must not enter the artifact"
        with self.assertRaises(review.ReviewError):
            review.validate_report(report)

    def test_extracts_fenced_json(self):
        content = "```json\n" + json.dumps(clean_report()) + "\n```"
        self.assertEqual(review.extract_json_object(content)["verdict"], "advisory_clear")


class RenderingTests(unittest.TestCase):
    def metadata(self):
        return {
            "head_sha": "a" * 40,
            "base_sha": "b" * 40,
            "provider": "Fireworks AI",
            "model": "accounts/fireworks/models/glm-5p3-flash",
            "reasoning_effort": "max",
            "harness_sha": "e" * 40,
            "prompt_sha256": "c" * 64,
            "packet_sha256": "d" * 64,
            "run_url": "https://github.com/example/project/actions/runs/1",
        }

    def test_renders_foundational_stop_and_marker(self):
        rendered = review.render_markdown(finding_report(foundational=True), self.metadata())
        self.assertIn(review.COMMENT_MARKER, rendered)
        self.assertIn("FOUNDATIONAL STOP", rendered)
        self.assertIn("No PR code was executed", rendered)

    def test_neutralizes_mentions(self):
        report = clean_report()
        report["summary"] = "Ask @maintainer."
        rendered = review.render_markdown(report, self.metadata())
        self.assertNotIn("@maintainer", rendered)

    def test_escapes_model_markdown_and_flattens_lines(self):
        report = clean_report()
        report["summary"] = "[click](https://evil.example)\n<img src=x>"
        rendered = review.render_markdown(report, self.metadata())
        self.assertNotIn("[click]", rendered)
        self.assertNotIn("<img", rendered)
        self.assertIn("&lt;img src=x&gt;", rendered)

    def test_compacts_a_schema_valid_oversized_report(self):
        report = finding_report()
        report["summary"] = "&" * 4000
        template = report["findings"][0]
        report["findings"] = []
        for index in range(50):
            finding = dict(template)
            finding["id"] = f"AR-{index + 1:03d}"
            finding["claim"] = "&" * 5000
            finding["requirement"] = "<" * 3000
            finding["reproduction"] = "*" * 5000
            finding["evidence"] = [
                {
                    "path": "p" * 500,
                    "line": "1" * 200,
                    "detail": ">" * 4000,
                }
                for _ in range(10)
            ]
            report["findings"].append(finding)
        report["findings"][-1]["severity"] = "P0"
        report["findings"][-1]["title"] = "Critical sentinel"
        report["limitations"] = ["&" * 2000 for _ in range(20)]

        review.validate_report(report)
        rendered = review.render_markdown(report, self.metadata())

        self.assertLessEqual(len(rendered), review.COMMENT_CHAR_LIMIT)
        self.assertIn("severity-prioritized bounded view", rendered)
        self.assertIn("42 additional finding(s)", rendered)
        self.assertIn("Critical sentinel", rendered)
        self.assertIn(review.COMMENT_MARKER, rendered)


class ConfigurationTests(unittest.TestCase):
    def test_loads_minimal_valid_config(self):
        config = {
            "schema_version": 1,
            "max_changed_files": 1,
            "max_diff_chars": 1,
            "max_changed_file_chars": 1,
            "max_contract_chars": 1,
            "contract_files": ["README.md"],
        }
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "config.json"
            path.write_text(json.dumps(config), encoding="utf-8")
            self.assertEqual(review.load_config(path)["contract_files"], ["README.md"])

    def test_rejects_parent_traversal(self):
        config = {
            "schema_version": 1,
            "max_changed_files": 1,
            "max_diff_chars": 1,
            "max_changed_file_chars": 1,
            "max_contract_chars": 1,
            "contract_files": ["../secret"],
        }
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "config.json"
            path.write_text(json.dumps(config), encoding="utf-8")
            with self.assertRaises(review.ReviewError):
                review.load_config(path)


class UtilityTests(unittest.TestCase):
    def test_bounded_marks_truncation(self):
        text, truncated = review.bounded("abcdef", 5)
        self.assertTrue(truncated)
        self.assertEqual(len(text), 5)

    def test_fireworks_url_is_restricted(self):
        with self.assertRaises(review.ReviewError):
            review.validate_fireworks_url("https://api.z.ai/api/paas/v4/chat/completions")

    def test_pinned_fireworks_url_is_allowed(self):
        self.assertEqual(
            review.validate_fireworks_url(review.FIREWORKS_API_URL),
            review.FIREWORKS_API_URL,
        )

    def test_fireworks_key_prefix_is_required(self):
        with self.assertRaises(review.ReviewError):
            review.validate_fireworks_api_key("not-a-fireworks-key")

    def test_verify_pr_shas_rejects_a_changed_head(self):
        current = {"base": {"sha": "b" * 40}, "head": {"sha": "n" * 40}}
        with (
            mock.patch.object(review, "github_json", return_value=current),
            self.assertRaises(review.ReviewError),
        ):
            review.verify_pr_shas(
                "example/project",
                7,
                "token",
                "b" * 40,
                "h" * 40,
            )

    def test_assemble_packet_rejects_a_fork(self):
        pull_request = {
            "base": {"sha": "b" * 40},
            "head": {
                "sha": "h" * 40,
                "repo": {"full_name": "fork-owner/project"},
            },
        }
        with (
            mock.patch.object(review, "github_json", return_value=pull_request),
            self.assertRaises(review.ReviewError),
        ):
            review.assemble_packet("example/project", 7, "token", {})

    def test_assemble_packet_uses_base_contract_and_head_file(self):
        pull_request = {
            "title": "Pilot",
            "body": "Body",
            "state": "open",
            "draft": False,
            "html_url": "https://github.com/example/project/pull/7",
            "user": {"login": "author"},
            "base": {"sha": "b" * 40, "ref": "main"},
            "head": {
                "sha": "h" * 40,
                "ref": "feature",
                "repo": {"full_name": "example/project"},
            },
        }
        changed = [
            {
                "filename": "src/lib.rs",
                "status": "modified",
                "additions": 1,
                "deletions": 1,
                "changes": 2,
            }
        ]

        def fake_github_json(_repository, path, _token, **_kwargs):
            if path == "/pulls/7":
                return pull_request
            if path.startswith("/pulls/7/files"):
                return changed
            self.fail(f"unexpected GitHub path: {path}")

        raw_calls = []

        def fake_github_raw(_repository, path, ref, _token):
            raw_calls.append((path, ref))
            return f"content of {path} at {ref}", None

        config = {
            "max_changed_files": 10,
            "max_diff_chars": 1000,
            "max_changed_file_chars": 1000,
            "max_contract_chars": 1000,
            "contract_files": ["CHARTER.md"],
        }
        with (
            mock.patch.object(review, "github_json", side_effect=fake_github_json),
            mock.patch.object(review, "github_raw", side_effect=fake_github_raw),
            mock.patch.object(review, "fetch_diff", return_value="diff"),
        ):
            packet = review.assemble_packet("example/project", 7, "token", config)

        self.assertIn(("CHARTER.md", "b" * 40), raw_calls)
        self.assertIn(("src/lib.rs", "h" * 40), raw_calls)
        self.assertFalse(packet["coverage"]["comments_and_prior_reviews_included"])
        self.assertFalse(packet["coverage"]["pull_request_code_executed"])

    def test_call_fireworks_requests_max_reasoning_and_structured_output(self):
        captured = {}

        def fake_http(_method, url, **kwargs):
            captured["url"] = url
            captured["payload"] = kwargs["payload"]
            response = {
                "choices": [
                    {"message": {"content": json.dumps(clean_report())}}
                ]
            }
            return json.dumps(response).encode("utf-8")

        with mock.patch.object(review, "_http_bytes", side_effect=fake_http):
            result = review.call_fireworks(
                review.FIREWORKS_API_URL,
                "fw_test-secret",
                review.FIREWORKS_MODEL,
                "max",
                "system prompt",
                {"packet_version": "0.1"},
            )

        self.assertEqual(result["verdict"], "advisory_clear")
        self.assertEqual(captured["payload"]["reasoning_effort"], "max")
        self.assertEqual(captured["payload"]["response_format"], {"type": "json_object"})
        self.assertNotIn("thinking", captured["payload"])
        self.assertEqual(captured["payload"]["model"], review.FIREWORKS_MODEL)
        self.assertEqual(captured["url"], review.FIREWORKS_API_URL)

    def test_main_writes_artifacts_and_publishes_validated_comment(self):
        config = {
            "schema_version": 1,
            "max_changed_files": 1,
            "max_diff_chars": 1,
            "max_changed_file_chars": 1,
            "max_contract_chars": 1,
            "contract_files": ["README.md"],
        }
        packet = {
            "packet_version": "0.1",
            "pull_request": {
                "base_sha": "b" * 40,
                "head_sha": "h" * 40,
            },
        }
        published = {}

        def fake_publish(repository, pr_number, token, body):
            published.update(
                repository=repository,
                pr_number=pr_number,
                token=token,
                body=body,
            )

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            config_path = root / "config.json"
            prompt_path = root / "prompt.md"
            output_path = root / "output"
            config_path.write_text(json.dumps(config), encoding="utf-8")
            prompt_path.write_text("System prompt", encoding="utf-8")
            environment = {
                "GITHUB_REPOSITORY": "example/project",
                "GITHUB_TOKEN": "github-secret",
                "GITHUB_RUN_ID": "123",
                "PR_NUMBER": "7",
                "REASONING_EFFORT": "max",
                "REVIEWER_CONFIG": str(config_path),
                "REVIEWER_PROMPT": str(prompt_path),
                "REVIEW_OUTPUT_DIR": str(output_path),
                "FIREWORKS_API_KEY": "fw_test-secret",
            }
            with (
                mock.patch.dict("os.environ", environment, clear=True),
                mock.patch.object(review, "assemble_packet", return_value=packet),
                mock.patch.object(review, "call_fireworks", return_value=clean_report()),
                mock.patch.object(review, "verify_pr_shas"),
                mock.patch.object(review, "publish_comment", side_effect=fake_publish),
            ):
                self.assertEqual(review.main(), 0)

            self.assertTrue((output_path / "review-packet.json").is_file())
            self.assertTrue((output_path / "review-result.json").is_file())
            self.assertTrue((output_path / "review-comment.md").is_file())
            self.assertEqual(published["repository"], "example/project")
            self.assertEqual(published["pr_number"], 7)
            self.assertIn(review.COMMENT_MARKER, published["body"])


if __name__ == "__main__":
    unittest.main()
