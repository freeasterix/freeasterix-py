from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="freeasterix",
    version="0.1.0",
    rust_extensions=[
        RustExtension("freeasterix.freeasterix", binding=Binding.PyO3)
    ],
    packages=["freeasterix"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
