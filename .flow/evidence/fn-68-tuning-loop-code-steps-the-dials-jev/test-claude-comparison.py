import importlib.util
import json
from pathlib import Path
import unittest

spec=importlib.util.spec_from_file_location("comparison",Path(__file__).with_name("claude-comparison.py"))
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)


class OfflineClaude(unittest.TestCase):
    def test_identical_payload_and_cached_accounting(self):
        request,payload=module.frozen_payload()
        self.assertEqual(len(payload["paths"]),5)
        self.assertEqual(module.digest(payload["prompt"].encode()),"61d328c368f40283b4ea1ddcc1e23ac03f620257515befa2874b5289477e7b70")
        answer={"passes":["unknown","unknown"],"defects":[],"observations":[],"findings":[]}
        result={"type":"result","subtype":"success","is_error":False,"structured_output":answer,
                "usage":{"input_tokens":10,"cache_creation_input_tokens":20,"cache_read_input_tokens":30,"output_tokens":4},
                "modelUsage":{"claude-fable-fixture":{}}}
        parsed=module.parse_result(json.dumps(result))
        self.assertEqual(parsed["actual_tokens"],64)
        self.assertEqual(parsed["actual_model"],"claude-fable-fixture")
        del result["structured_output"]
        assistant={"type":"assistant","message":{"content":[{"type":"text","text":json.dumps(answer)}]}}
        self.assertEqual(module.parse_result(json.dumps(assistant)+"\n"+json.dumps(result))["answer"],answer)
        del result["usage"]["cache_read_input_tokens"]
        with self.assertRaises(ValueError):module.parse_result(json.dumps(result))
        result["subtype"]="error_max_turns"
        with self.assertRaises(ValueError):module.parse_result(json.dumps(result))


if __name__=="__main__":unittest.main()
