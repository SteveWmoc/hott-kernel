import io
import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import adversarial_review as review


def raw_usage(*, prompt_tokens=1000, cached_tokens=100, completion_tokens=2000):
    return {
        "prompt_tokens": prompt_tokens,
        "completion_tokens": completion_tokens,
        "total_tokens": prompt_tokens + completion_tokens,
        "prompt_tokens_details": {"cached_tokens": cached_tokens},
    }


def usage_record():
    return review.build_usage_record(raw_usage())


def minimal_config(*, contract_files=None):
    return {
        "schema_version": 2,
        "max_changed_files": 1,
        "max_diff_chars": 1,
        "max_changed_file_chars": 1,
        "max_contract_chars": 1,
        "contract_files": contract_files or ["README.md"],
        "profile_routes": [
            {"profile": "ci-supply-chain", "patterns": [".github/**"]}
        ],
    }


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
            "current_head_sha": "a" * 40,
            "base_sha": "b" * 40,
            "review_mode": "current",
            "review_phase": "code",
            "requested_profile": "broad",
            "review_profile": "broad",
            "matched_profiles": [],
            "provider": "Fireworks AI",
            "model": "accounts/fireworks/models/glm-5p3-flash",
            "reasoning_effort": "max",
            "harness_sha": "e" * 40,
            "prompt_sha256": "c" * 64,
            "packet_sha256": "d" * 64,
            "usage": usage_record(),
            "recorded_at_utc": "2026-09-01T15:00:00Z",
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

    def test_historical_replay_has_a_distinct_marker_and_warning(self):
        metadata = self.metadata()
        metadata.update(
            head_sha="c" * 40,
            current_head_sha="a" * 40,
            review_mode="historical",
        )
        rendered = review.render_markdown(clean_report(), metadata)

        self.assertIn("historical-head=" + "c" * 40, rendered)
        self.assertNotIn(review.COMMENT_MARKER + "\n", rendered)
        self.assertIn("Historical adversarial-review replay", rendered)
        self.assertIn("not a review of the pull request's current or final head", rendered)
        self.assertIn("Current/final PR head", rendered)

    def test_focused_replay_has_a_profile_specific_identity(self):
        metadata = self.metadata()
        metadata.update(
            head_sha="c" * 40,
            current_head_sha="a" * 40,
            review_mode="historical",
            review_profile="schema-encoding",
        )
        rendered = review.render_markdown(clean_report(), metadata)

        self.assertIn("profile=schema-encoding", rendered)
        self.assertIn("mode=historical", rendered)
        self.assertIn("Historical focused schema-and-encoding replay", rendered)
        self.assertIn("Resolved profile | `schema-encoding`", rendered)
        self.assertNotIn(review.comment_marker_for("historical", "c" * 40) + "\n", rendered)

    def test_design_review_has_a_distinct_marker_and_heading(self):
        metadata = self.metadata()
        metadata.update(
            review_phase="design",
            requested_profile="auto",
            review_profile="foundational-consistency",
            matched_profiles=["foundational-consistency"],
        )

        rendered = review.render_markdown(clean_report(), metadata)

        self.assertIn("phase=design", rendered)
        self.assertIn("Focused foundational-consistency design review", rendered)
        self.assertIn("Review phase | `design`", rendered)
        self.assertIn("Auto-route matches | `foundational-consistency`", rendered)

    def test_renders_token_usage_and_cost_estimate(self):
        rendered = review.render_markdown(clean_report(), self.metadata())

        self.assertIn("Prompt tokens | 1000", rendered)
        self.assertIn("Cached prompt tokens | 100", rendered)
        self.assertIn("Completion tokens | 2000", rendered)
        self.assertIn("Estimated cost | $0.001138 USD", rendered)

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
        config = minimal_config()
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "config.json"
            path.write_text(json.dumps(config), encoding="utf-8")
            self.assertEqual(review.load_config(path)["contract_files"], ["README.md"])

    def test_checked_in_config_routes_every_focused_profile(self):
        root = Path(__file__).resolve().parent.parent
        config = review.load_config(root / ".github/adversarial-review/config.json")

        routed = {route["profile"] for route in config["profile_routes"]}
        self.assertEqual(routed, set(review.REVIEW_PROFILE_FILES) - {"broad"})

    def test_rejects_parent_traversal(self):
        config = minimal_config(contract_files=["../secret"])
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "config.json"
            path.write_text(json.dumps(config), encoding="utf-8")
            with self.assertRaises(review.ReviewError):
                review.load_config(path)

    def test_composes_a_pinned_focused_prompt(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base.md"
            phase = root / review.REVIEW_PHASE_FILES["code"]
            focused = root / review.REVIEW_PROFILE_FILES["schema-encoding"]
            focused.parent.mkdir(parents=True)
            base.write_text("Base contract\n", encoding="utf-8")
            phase.parent.mkdir(parents=True, exist_ok=True)
            phase.write_text("Code contract\n", encoding="utf-8")
            focused.write_text("Focused contract\n", encoding="utf-8")

            prompt = review.compose_review_prompt(root, base, "code", "schema-encoding")

        self.assertEqual(
            prompt,
            "Base contract\n\nCode contract\n\nFocused contract\n",
        )

    def test_rejects_an_unknown_review_profile(self):
        with self.assertRaises(review.ReviewError):
            review.validate_review_profile("untrusted-profile")

    def test_auto_route_uses_checked_in_risk_priority(self):
        routes = [
            {"profile": "foundational-consistency", "patterns": ["docs/decisions/**"]},
            {"profile": "kernel-soundness", "patterns": ["src/kernel/**"]},
        ]

        resolved, matches = review.resolve_review_profile(
            "auto",
            ["src/kernel/check.rs", "docs/decisions/0013.md"],
            routes,
        )

        self.assertEqual(resolved, "foundational-consistency")
        self.assertEqual(matches, ["foundational-consistency", "kernel-soundness"])

    def test_auto_route_falls_back_to_broad(self):
        resolved, matches = review.resolve_review_profile(
            "auto",
            ["README.md"],
            [{"profile": "parser-boundary", "patterns": ["src/format/**"]}],
        )

        self.assertEqual((resolved, matches), ("broad", []))

    def test_auto_route_rejects_a_truncated_changed_file_list(self):
        packet = {
            "coverage": {"changed_file_list_truncated": True},
            "changed_files": [{"path": "src/format/parser.rs"}],
        }
        with self.assertRaises(review.ReviewError):
            review.resolve_packet_review_profile(
                "auto",
                packet,
                [{"profile": "parser-boundary", "patterns": ["src/format/**"]}],
            )

    def test_explicit_profile_bypasses_path_routing(self):
        resolved, matches = review.resolve_review_profile(
            "kernel-soundness",
            ["schemas/foundation.json"],
            [{"profile": "schema-encoding", "patterns": ["schemas/**"]}],
        )

        self.assertEqual((resolved, matches), ("kernel-soundness", []))

    def test_rejects_unknown_review_phase(self):
        with self.assertRaises(review.ReviewError):
            review.validate_review_phase("deployment")

    def test_base_prompt_pins_calibration_safeguards(self):
        root = Path(__file__).resolve().parent.parent
        prompt = (root / ".github/adversarial-review/prompt.md").read_text(
            encoding="utf-8"
        )

        self.assertIn("never merely `P3`", prompt)
        self.assertIn("the proposed remedy changes a frozen decision", prompt)
        self.assertIn("consistency check across `verdict`, `summary`", prompt)


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

    def test_historical_head_must_belong_to_the_pull_request(self):
        with (
            mock.patch.object(review, "list_pr_commit_shas", return_value={"c" * 40}),
            self.assertRaises(review.ReviewError),
        ):
            review.select_review_head_sha(
                "example/project",
                7,
                "token",
                "a" * 40,
                "b" * 40,
            )

    def test_selects_a_historical_pr_commit(self):
        with mock.patch.object(
            review,
            "list_pr_commit_shas",
            return_value={"b" * 40},
        ):
            self.assertEqual(
                review.select_review_head_sha(
                    "example/project",
                    7,
                    "token",
                    "a" * 40,
                    "b" * 40,
                ),
                ("b" * 40, "historical"),
            )

    def test_historical_comment_does_not_replace_the_current_head_review(self):
        calls = []

        def fake_github_json(_repository, path, _token, **kwargs):
            calls.append((path, kwargs))
            if path.startswith("/issues/7/comments?"):
                return [
                    {
                        "id": 1,
                        "body": review.COMMENT_MARKER + "\ncurrent review",
                        "user": {"login": "github-actions[bot]"},
                    }
                ]
            return {}

        marker = review.comment_marker_for("historical", "b" * 40)
        with mock.patch.object(review, "github_json", side_effect=fake_github_json):
            review.publish_comment(
                "example/project",
                7,
                "token",
                marker,
                marker + "\nhistorical review",
            )

        self.assertTrue(any(path == "/issues/7/comments" for path, _kwargs in calls))
        self.assertFalse(any(path == "/issues/comments/1" for path, _kwargs in calls))

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
                "sha": "a" * 40,
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
                "sha": "a" * 40,
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
        self.assertIn(("src/lib.rs", "a" * 40), raw_calls)
        self.assertEqual(packet["pull_request"]["review_mode"], "current")
        self.assertEqual(packet["pull_request"]["current_head_sha"], "a" * 40)
        self.assertFalse(packet["coverage"]["comments_and_prior_reviews_included"])
        self.assertFalse(packet["coverage"]["pull_request_code_executed"])

    def test_assemble_packet_uses_a_historical_comparison(self):
        pull_request = {
            "title": "Pilot",
            "body": "Body",
            "state": "closed",
            "draft": False,
            "html_url": "https://github.com/example/project/pull/7",
            "user": {"login": "author"},
            "base": {"sha": "b" * 40, "ref": "main"},
            "head": {
                "sha": "c" * 40,
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
            if path.startswith("/pulls/7/commits"):
                return [{"sha": "a" * 40}]
            if path == f"/compare/{'b' * 40}...{'a' * 40}":
                return {
                    "merge_base_commit": {"sha": "b" * 40},
                    "files": changed,
                }
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
            mock.patch.object(review, "fetch_comparison_diff", return_value="historical diff"),
        ):
            packet = review.assemble_packet(
                "example/project",
                7,
                "token",
                config,
                requested_head_sha="a" * 40,
            )

        self.assertIn(("CHARTER.md", "b" * 40), raw_calls)
        self.assertIn(("src/lib.rs", "a" * 40), raw_calls)
        self.assertEqual(packet["packet_version"], "0.2")
        self.assertEqual(packet["pull_request"]["head_sha"], "a" * 40)
        self.assertEqual(packet["pull_request"]["current_head_sha"], "c" * 40)
        self.assertEqual(packet["pull_request"]["review_mode"], "historical")
        self.assertTrue(packet["coverage"]["historical_replay"])
        self.assertEqual(packet["unified_diff"], "historical diff")

    def test_call_fireworks_requests_max_reasoning_and_structured_output(self):
        captured = {}

        def fake_stream(url, _api_key, payload):
            captured["url"] = url
            captured["payload"] = payload
            return json.dumps(clean_report()), raw_usage()

        with mock.patch.object(review, "_fireworks_stream_content_once", side_effect=fake_stream):
            result, usage = review.call_fireworks(
                review.FIREWORKS_API_URL,
                "fw_test-secret",
                review.FIREWORKS_MODEL,
                "max",
                "system prompt",
                {"packet_version": "0.1"},
            )

        self.assertEqual(result["verdict"], "advisory_clear")
        self.assertEqual(usage["prompt_tokens"], 1000)
        self.assertEqual(captured["payload"]["reasoning_effort"], "max")
        self.assertEqual(captured["payload"]["response_format"], {"type": "json_object"})
        self.assertEqual(captured["payload"]["max_tokens"], review.FIREWORKS_MAX_TOKENS)
        self.assertNotIn("thinking", captured["payload"])
        self.assertTrue(captured["payload"]["stream"])
        self.assertFalse(
            captured["payload"]["stream_options"]["include_internal_content"]
        )
        self.assertEqual(captured["payload"]["stream_options"]["buffer_ms"], 1000)
        self.assertEqual(captured["payload"]["model"], review.FIREWORKS_MODEL)
        self.assertEqual(captured["url"], review.FIREWORKS_API_URL)

    def test_fireworks_stream_ignores_reasoning_and_collects_content(self):
        report_text = json.dumps(clean_report())
        midpoint = len(report_text) // 2
        events = [
            {
                "choices": [
                    {"delta": {"reasoning_content": "private reasoning"}, "finish_reason": None}
                ]
            },
            {
                "choices": [
                    {"delta": {"content": report_text[:midpoint]}, "finish_reason": None}
                ]
            },
            {
                "choices": [
                    {"delta": {"content": report_text[midpoint:]}, "finish_reason": "stop"}
                ]
            },
            {"choices": [], "usage": raw_usage()},
        ]
        stream = b"".join(
            f"data: {json.dumps(event)}\n\n".encode("utf-8") for event in events
        ) + b"data: [DONE]\n\n"

        with mock.patch.object(review.urllib.request, "urlopen", return_value=io.BytesIO(stream)):
            content, streamed_usage = review._fireworks_stream_content_once(
                review.FIREWORKS_API_URL,
                "fw_test-secret",
                {"stream": True},
            )

        self.assertEqual(content, report_text)
        self.assertNotIn("private reasoning", content)
        self.assertEqual(streamed_usage, raw_usage())

    def test_fireworks_stream_retries_once_before_output(self):
        with (
            mock.patch.object(
                review,
                "_fireworks_stream_content_once",
                side_effect=[
                    review.TransientReviewError("temporary disconnect"),
                    (json.dumps(clean_report()), raw_usage()),
                ],
            ) as streamed,
            mock.patch.object(review.time, "sleep") as sleep,
        ):
            result, usage = review.call_fireworks(
                review.FIREWORKS_API_URL,
                "fw_test-secret",
                review.FIREWORKS_MODEL,
                "max",
                "system prompt",
                {"packet_version": "0.1"},
            )

        self.assertEqual(result["verdict"], "advisory_clear")
        self.assertEqual(usage["completion_tokens"], 2000)
        self.assertEqual(streamed.call_count, 2)
        sleep.assert_called_once_with(2)

    def test_fireworks_stream_rejects_incomplete_output(self):
        stream = (
            b'data: {"choices":[{"delta":{"reasoning_content":"partial"},'
            b'"finish_reason":null}]}\n\n'
        )
        with (
            mock.patch.object(review.urllib.request, "urlopen", return_value=io.BytesIO(stream)),
            self.assertRaises(review.ReviewError),
        ):
            review._fireworks_stream_content_once(
                review.FIREWORKS_API_URL,
                "fw_test-secret",
                {"stream": True},
            )

    def test_fireworks_stream_rejects_complete_output_without_usage(self):
        event = {
            "choices": [
                {
                    "delta": {"content": json.dumps(clean_report())},
                    "finish_reason": "stop",
                }
            ]
        }
        stream = (
            f"data: {json.dumps(event)}\n\ndata: [DONE]\n\n".encode("utf-8")
        )
        with (
            mock.patch.object(review.urllib.request, "urlopen", return_value=io.BytesIO(stream)),
            self.assertRaises(review.ReviewError),
        ):
            review._fireworks_stream_content_once(
                review.FIREWORKS_API_URL,
                "fw_test-secret",
                {"stream": True},
            )

    def test_usage_record_accounts_for_cached_prompt_tokens(self):
        usage = review.build_usage_record(
            raw_usage(prompt_tokens=1_000_000, cached_tokens=250_000, completion_tokens=100_000)
        )

        self.assertEqual(usage["uncached_prompt_tokens"], 750_000)
        self.assertEqual(usage["estimated_cost_usd"], "0.170000")
        self.assertEqual(
            usage["rates_usd_per_million_tokens"],
            {"input": "0.15", "cached_input": "0.03", "output": "0.50"},
        )

    def test_usage_record_rejects_an_inconsistent_total(self):
        usage = raw_usage()
        usage["total_tokens"] += 1
        with self.assertRaises(review.ReviewError):
            review.build_usage_record(usage)

    def test_main_writes_artifacts_and_publishes_validated_comment(self):
        config = minimal_config()
        packet = {
            "packet_version": "0.1",
            "changed_files": [],
            "pull_request": {
                "base_sha": "b" * 40,
                "head_sha": "h" * 40,
                "current_head_sha": "h" * 40,
                "review_mode": "current",
            },
        }
        published = {}

        def fake_publish(repository, pr_number, token, marker, body):
            published.update(
                repository=repository,
                pr_number=pr_number,
                token=token,
                marker=marker,
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
                "REVIEW_PHASE": "code",
                "REVIEW_PROFILE": "broad",
                "REASONING_EFFORT": "max",
                "REVIEWER_CONFIG": str(config_path),
                "REVIEWER_PROMPT": str(prompt_path),
                "REVIEW_OUTPUT_DIR": str(output_path),
                "FIREWORKS_API_KEY": "fw_test-secret",
            }
            with (
                mock.patch.dict("os.environ", environment, clear=True),
                mock.patch.object(review, "assemble_packet", return_value=packet),
                mock.patch.object(
                    review,
                    "call_fireworks",
                    return_value=(clean_report(), usage_record()),
                ),
                mock.patch.object(review, "verify_pr_shas"),
                mock.patch.object(review, "publish_comment", side_effect=fake_publish),
            ):
                self.assertEqual(review.main(), 0)

            self.assertTrue((output_path / "review-packet.json").is_file())
            self.assertTrue((output_path / "review-result.json").is_file())
            self.assertTrue((output_path / "review-comment.md").is_file())
            self.assertTrue((output_path / "review-usage.json").is_file())
            usage_audit = json.loads(
                (output_path / "review-usage.json").read_text(encoding="utf-8")
            )
            self.assertEqual(usage_audit["resolved_profile"], "broad")
            self.assertEqual(usage_audit["usage"]["estimated_cost_usd"], "0.001138")
            self.assertEqual(usage_audit["reviewed_head_sha"], "h" * 40)
            self.assertEqual(published["repository"], "example/project")
            self.assertEqual(published["pr_number"], 7)
            self.assertEqual(published["marker"], review.COMMENT_MARKER)
            self.assertIn(review.COMMENT_MARKER, published["body"])


if __name__ == "__main__":
    unittest.main()
