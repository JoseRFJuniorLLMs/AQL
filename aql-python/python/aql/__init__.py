"""
AQL — Agent Query Language v2.0

A cognitive intent language for agents interacting with
knowledge graphs and vector databases.

The Three Axioms:
1. Intention as Primitive — The atomic unit is a cognitive act.
2. Uncertainty as Data Type — CONFIDENCE is epistemic, not a filter.
3. Effects as Automatic Consequences — Declare intent; server applies side-effects.

Usage:
    import aql

    # Parse AQL
    ast = aql.parse_aql('RECALL "quantum physics" CONFIDENCE 0.8')

    # Plan execution
    plans = aql.plan_aql('RECALL "quantum" THEN DISTILL @results')

    # List verbs
    print(aql.verbs())  # ['RECALL', 'RESONATE', ...]
"""

from .aql import parse_aql, plan_aql, version, verbs, epistemic_types

__version__ = version()
__all__ = ["parse_aql", "plan_aql", "version", "verbs", "epistemic_types"]
