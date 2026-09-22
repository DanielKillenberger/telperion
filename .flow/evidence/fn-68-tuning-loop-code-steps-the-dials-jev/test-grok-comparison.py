import importlib.util,json,unittest
from pathlib import Path
s=importlib.util.spec_from_file_location("g",Path(__file__).with_name("grok-comparison.py"));m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
class OfflineGrok(unittest.TestCase):
 def test_usage_and_no_extra_models(self):
  r={"stopReason":"end_turn","num_turns":1,"usage":{"input_tokens":10,"cache_read_input_tokens":20,"cache_creation_input_tokens":3,"output_tokens":7,"reasoning_tokens":5,"total_tokens":40},"modelUsage":{"grok-4.6":{"modelCalls":1}},"structuredOutput":{"passes":["fail","fail"],"defects":[],"observations":[],"findings":[]}}
  self.assertEqual(m.parse_result(json.dumps(r))["actual_tokens"],40)
  r["modelUsage"]["caption-model"]={"modelCalls":1}
  with self.assertRaises(ValueError):m.parse_result(json.dumps(r))
  del r["modelUsage"]["caption-model"];del r["usage"]["total_tokens"]
  with self.assertRaises(ValueError):m.parse_result(json.dumps(r))
if __name__=="__main__":unittest.main()
