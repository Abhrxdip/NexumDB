"""
NexumDB AI Engine
Handles embedding generation, semantic caching, and query optimization
"""

__version__ = "0.1.0"

from .optimizer import SemanticCache, QueryOptimizer

__all__ = ["SemanticCache", "QueryOptimizer"]
