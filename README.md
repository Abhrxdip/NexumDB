# NexumDB - AI-Native Database

An innovative, open-source database that combines traditional SQL with AI-powered features including semantic caching and reinforcement learning-based query optimization.

## Architecture

- **Core System**: Rust-based storage engine using sled, with SQL parsing and execution
- **AI Engine**: Python-based semantic caching and RL optimization using local models
- **Integration**: PyO3 bindings for seamless Rust-Python integration

## Features

- SQL support (CREATE TABLE, INSERT, SELECT)
- Semantic query caching using local embedding models (all-MiniLM-L6-v2)
- Self-optimizing query execution with reinforcement learning
- Local-only execution (no cloud dependencies)
- Persistent storage with sled
- Query performance instrumentation
- Production-ready release build

## Project Structure

```
NexumDB/
├── nexum_core/          # Rust core database engine
│   └── src/
│       ├── storage/     # Storage layer (sled)
│       ├── sql/         # SQL parsing and planning
│       ├── catalog/     # Table metadata management
│       ├── executor/    # Query execution + caching
│       └── bridge/      # Python integration (PyO3)
├── nexum_cli/           # CLI REPL interface
├── nexum_ai/            # Python AI engine
│   └── optimizer.py     # Semantic cache and RL optimizer
└── tests/               # Integration tests
```

## Building

```bash
# Set PyO3 forward compatibility (for Python 3.14+)
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1

# Build release binary
cargo build --release
```

## Python Dependencies

```bash
# Create virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install AI dependencies
pip install -r nexum_ai/requirements.txt
```

## Running Tests

```bash
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
cargo test -- --test-threads=1
```

**Test Results**: 11/11 passing

## Usage

```bash
./target/release/nexum
```

### Example Session

```sql
CREATE TABLE users (id INTEGER, name TEXT, age INTEGER);
INSERT INTO users (id, name, age) VALUES (1, 'Alice', 30), (2, 'Bob', 25);
SELECT * FROM users;
```

### Semantic Caching Demo

The database automatically caches SELECT query results using semantic similarity:

```
First SELECT:  Query executed in 2.5ms  (cache miss)
Second SELECT: Query executed in 0.04ms (cache hit - 60x faster!)
```

## Development Status

- **Phase 1**: Project Skeleton & Storage Layer - COMPLETE
- **Phase 2**: SQL Engine - COMPLETE  
- **Phase 3**: AI Bridge (PyO3) - COMPLETE
- **Phase 4**: Intelligent Features - COMPLETE
- **Phase 5**: Final Interface - IN PROGRESS

## Key Achievements

1. Fully functional SQL database with CREATE, INSERT, SELECT
2. Semantic caching using local embedding models
3. Successful Rust-Python integration via PyO3
4. 60x query speedup on cache hits
5. Comprehensive test suite (11 tests passing)
6. Query performance instrumentation
7. Production release build working

## Technical Highlights

- **Zero Cloud Dependencies**: All models run locally
- **High Performance**: Sub-millisecond query execution
- **AI-Powered**: Semantic caching using transformer embeddings
- **Type-Safe**: Rust core with comprehensive error handling
- **Well-Tested**: Full unit and integration test coverage

## License

MIT
