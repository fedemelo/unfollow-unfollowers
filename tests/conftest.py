import pytest


@pytest.fixture
def export_dir(tmp_path):
    path = tmp_path / "export"
    path.mkdir()
    return path
