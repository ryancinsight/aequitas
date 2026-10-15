"""Pin the Aequitas caller's validation and explicit publication contract."""

from __future__ import annotations

import unittest
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github/workflows/book-pages.yml"
PUBLICATION = (
    "${{ github.event_name == 'workflow_dispatch' "
    "&& github.ref == 'refs/heads/main' && inputs.publish }}"
)


class BookPagesWorkflowTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.workflow = yaml.safe_load(WORKFLOW.read_text(encoding="utf-8"))
        cls.events = cls.workflow[True]
        cls.caller = cls.workflow["jobs"]["deploy"]

    def test_dependency_and_source_changes_still_validate_on_push_and_pull_request(self) -> None:
        self.assertEqual(set(self.events), {"push", "pull_request", "workflow_dispatch"})
        self.assertEqual(self.events["push"]["branches"], ["main"])
        paths = {
            "docs/book/**", "src/**", "Cargo.toml", "Cargo.lock",
            ".github/workflows/book-pages.yml", "scripts/tests/test_book_pages_workflow.py",
        }
        for event in ("push", "pull_request"):
            with self.subTest(event=event):
                self.assertEqual(set(self.events[event]["paths"]), paths)

    def test_manual_publication_requires_an_explicit_boolean_opt_in(self) -> None:
        contract = self.events["workflow_dispatch"]["inputs"]["publish"]
        self.assertEqual(contract["type"], "boolean")
        self.assertIs(contract["required"], False)
        self.assertIs(contract["default"], False)

    def test_only_an_opted_in_main_dispatch_authorizes_publication(self) -> None:
        self.assertEqual(self.caller["with"].get("deploy"), PUBLICATION)
        self.assertNotIn("if", self.caller)
        self.assertNotIn("continue-on-error", self.caller)

    def test_book_compilation_and_sample_testing_are_preserved(self) -> None:
        supplied = self.caller["with"]
        self.assertEqual(supplied, {
            "output-path": "target/book/aequitas", "mdbook-test": True,
            "rust-toolchain": "1.97.0", "cargo-package": "aequitas",
            "deploy": PUBLICATION,
        })
        self.assertRegex(
            self.caller["uses"],
            r"^ryancinsight/atlas/\.github/workflows/book-pages\.yml@[0-9a-f]{40}$",
        )
        self.assertNotEqual(
            self.caller["uses"].rsplit("@", 1)[1],
            "20c93980f7c98f2e23a89c4a0540f16c8f2d7239",
            "The old workflow does not accept the deploy input",
        )

    def test_permissions_and_deployment_serialization_are_preserved(self) -> None:
        self.assertEqual(self.workflow["name"], "Deploy mdBook")
        self.assertEqual(self.workflow["permissions"], {"contents": "read"})
        self.assertEqual(self.caller["permissions"], {
            "contents": "read", "pages": "write", "id-token": "write",
        })
        self.assertEqual(self.workflow["concurrency"], {
            "group": "book-pages-${{ github.ref }}",
            "cancel-in-progress": "${{ github.event_name == 'pull_request' }}",
        })

    def test_contract_validation_precedes_every_book_run(self) -> None:
        self.assertEqual(self.caller["needs"], "contract")
        contract = self.workflow["jobs"]["contract"]
        self.assertNotIn("if", contract)
        self.assertNotIn("continue-on-error", contract)
        self.assertEqual(contract["timeout-minutes"], 5)
        self.assertEqual(contract["steps"][-1]["run"],
                         "python3 -m unittest discover -s scripts/tests -p 'test_book_pages_workflow.py' -v")


if __name__ == "__main__":
    unittest.main()
