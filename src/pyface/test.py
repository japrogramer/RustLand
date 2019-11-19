import unittest

from .baseline import read_data

class TestModule(unittest.TestCase):
    def test_read_data(self):
        """TODO: Docstring for test_read_data.

        :arg1: TODO
        :returns: TODO

        """
        v = read_data(1)
        self.assertEqual(v, '{}'.format([i for i in range(10)]))
