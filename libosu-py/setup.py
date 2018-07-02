from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="libosu",
    version="0.0.2",
    description="General osu! library",
    license="MIT",

    rust_extensions=[RustExtension('libosu', 'Cargo.toml', binding=Binding.PyO3)],
    packages=['libosu'],
    zip_safe=False,
)

