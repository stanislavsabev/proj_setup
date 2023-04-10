"""Test start module."""

from python_proj import start


def test_start_here():
    """Test start.here."""
    expected = 'Your code goes here!'
    actual = start.here()
    assert actual == expected
