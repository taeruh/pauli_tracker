import os

project = "pauli_tracker"
copyright = "2023, Jannis Ruh"
author = "Jannis Ruh"
release = "0.1.0"

html_static_path = ["_static"]
templates_path = ["_templates"]
# exclude_patterns = ["_build", "_templates", "_static"]

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.viewcode",
    "sphinx.ext.napoleon",
    "sphinx_autodoc_typehints",
    "sphinx_rtd_theme",
]

autosummary_generate = True
# don't import any class or function directly except the ones from the "Rust backend
# module"
autosummary_imported_members = True
# autoclass_content = "both"
html_show_sourcelink = False

on_rtd = os.environ.get("READTHEDOCS", None) == "True"
if not on_rtd:
    import sphinx_rtd_theme

    html_theme = "sphinx_rtd_theme"
    html_theme_path = [sphinx_rtd_theme.get_html_theme_path()]
html_css_files = ["custom_readthedocs.css"]
