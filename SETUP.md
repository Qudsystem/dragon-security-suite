# Protect X - Setup Guide

## Step-by-Step Installation

### 1. Install Rust (Required)

#### Windows
```powershell
# Download and run rustup-init.exe
# From: https://win.rustup.rs/

# Or using winget
winget install Rustlang.Rustup

# Verify installation
rustc --version
cargo --version
```

#### Linux/macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2. Install Python 3.11+ (Required)

#### Windows
```powershell
# Download from python.org or use winget
winget install Python.Python.3.11

# Verify installation
python --version
```

#### Linux
```bash
sudo apt update
sudo apt install python3.11 python3.11-venv python3-pip

# Verify installation
python3.11 --version
```

### 3. Install SurrealDB (Required)

#### Windows
```powershell
# Download from https://surrealdb.com/install
# Or use PowerShell script
iwr https://windows.surrealdb.com -useb | iex

# Verify installation
surreal version
```

#### Linux
```bash
curl -sSf https://install.surrealdb.com | sh

# Verify installation
surreal version
```

### 4. Install Docker (Optional, for containerization)

#### Windows
Download Docker Desktop from: https://www.docker.com/products/docker-desktop/

#### Linux
```bash
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
```

### 5. Setup Project

```bash
# Clone or navigate to project directory
cd protect-x

# Setup Rust core
cd rust-core
cargo build
cargo test

# Setup Python AI layer
cd ../python-ai
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r requirements.txt

# Setup UI
cd ../ui
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r requirements.txt
```

### 6. Download MITRE ATT&CK Data

```bash
# Create data directory
mkdir -p data/mitre_attack

# Download MITRE ATT&CK framework
cd data/mitre_attack
curl -O https://raw.githubusercontent.com/mitre/cti/master/enterprise-attack/enterprise-attack.json
```

### 7. Start Development Environment

#### Terminal 1: SurrealDB
```bash
surreal start --log trace --user root --pass root memory
```

#### Terminal 2: Rust Core
```bash
cd rust-core
cargo run
```

#### Terminal 3: Python AI Layer
```bash
cd python-ai
source venv/bin/activate
python main.py
```

#### Terminal 4: Streamlit UI
```bash
cd ui
source venv/bin/activate
streamlit run app.py
```

### 8. Access the Application

Open your browser and navigate to:
- **UI Dashboard**: http://localhost:8501
- **API**: http://localhost:8000 (when implemented)

## Troubleshooting

### Rust Installation Issues

**Problem**: `rustc` command not found
**Solution**: 
```bash
# Add Rust to PATH
source $HOME/.cargo/env

# Or on Windows, restart your terminal
```

### Python Virtual Environment Issues

**Problem**: Cannot activate virtual environment
**Solution**:
```bash
# Linux/macOS
python3 -m venv venv
source venv/bin/activate

# Windows PowerShell
python -m venv venv
.\venv\Scripts\Activate.ps1

# Windows CMD
python -m venv venv
venv\Scripts\activate.bat
```

### SurrealDB Connection Issues

**Problem**: Cannot connect to SurrealDB
**Solution**:
```bash
# Check if SurrealDB is running
surreal version

# Start SurrealDB with correct parameters
surreal start --log trace --user root --pass root memory

# Check connection
curl http://localhost:8000/health
```

### Port Already in Use

**Problem**: Port 8501 or 50051 already in use
**Solution**:
```bash
# Find process using port (Linux/macOS)
lsof -i :8501
kill -9 <PID>

# Windows
netstat -ano | findstr :8501
taskkill /PID <PID> /F
```

## Next Steps

1. Read the [Requirements](/.kiro/specs/protect-x/requirements.md)
2. Review the [Design](/.kiro/specs/protect-x/design.md)
3. Follow the [Implementation Plan](/.kiro/specs/protect-x/tasks.md)
4. Start implementing tasks from `tasks.md`

## Development Workflow

1. Pick a task from `tasks.md`
2. Implement the feature
3. Write tests (unit + property-based)
4. Run tests: `cargo test` or `pytest`
5. Commit changes
6. Move to next task

## Testing

```bash
# Rust tests
cd rust-core
cargo test                    # Unit tests
cargo test --test proptest    # Property-based tests
cargo bench                   # Benchmarks

# Python tests
cd python-ai
pytest                        # All tests
pytest tests/property_tests/  # Property-based tests
pytest --cov=.                # With coverage
```

## Performance Monitoring

```bash
# Rust profiling
cd rust-core
cargo build --release
cargo flamegraph

# Python profiling
cd python-ai
python -m cProfile main.py
```

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review the specification documents
3. Check the GitHub issues (when available)
