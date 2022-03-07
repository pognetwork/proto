import pathlib
from setuptools import setup, find_packages

# The directory containing this file
HERE = pathlib.Path(__file__).parent

# The text of the README file
README = (HERE / "README.md").read_text()

# This call to setup() does all the work
setup(
    name="pog-proto",
    version="1.0.0",
    description="This repository contains protocol buffers which are shared across pog projects and can be used to interact with the different APIs safely.",
    long_description=README,
    long_description_content_type="text/markdown",
    url="https://github.com/pognetwork/proto",
    author="pog.network contributors",
    author_email="team@pog.network",
    license="MIT",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
    ],
    install_requires=[
        'protobuf>=3.19.4',
    ],
    packages=["pog_proto"],
    include_package_data=True,
)
