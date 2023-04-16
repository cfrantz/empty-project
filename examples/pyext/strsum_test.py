import unittest
from examples.pyext import strsum

class TestStrSumModule(unittest.TestCase):

    def test_sum_as_string(self):
        result = strsum.sum_as_string(3, 5)
        self.assertEqual(result, '8')

if __name__ == '__main__':
    unittest.main()
